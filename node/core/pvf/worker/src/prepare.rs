// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#[cfg(target_os = "linux")]
use crate::memory_stats::max_rss_stat::{extract_max_rss_stat, get_max_rss_thread};
#[cfg(any(target_os = "linux", feature = "jemalloc-allocator"))]
use crate::memory_stats::memory_tracker::{get_memory_tracker_loop_stats, memory_tracker_loop};
use crate::{
	common::{bytes_to_path, cpu_time_monitor_loop, stringify_panic_payload, worker_event_loop},
	prepare, prevalidate, LOG_TARGET,
};
use cpu_time::ProcessTime;
use parity_scale_codec::{Decode, Encode};
use polkadot_node_core_pvf::{
	framed_recv, framed_send, CompiledArtifact, MemoryStats, PrepareError, PrepareResult,
	PrepareStats, PvfPrepData,
};
use std::{panic, path::PathBuf, sync::mpsc::channel, thread, time::Duration};
use tokio::{io, net::UnixStream};

async fn recv_request(stream: &mut UnixStream) -> io::Result<(PvfPrepData, PathBuf)> {
	let pvf = framed_recv(stream).await?;
	let pvf = PvfPrepData::decode(&mut &pvf[..]).map_err(|e| {
		io::Error::new(
			io::ErrorKind::Other,
			format!("prepare pvf recv_request: failed to decode PvfPrepData: {}", e),
		)
	})?;
	let tmp_file = framed_recv(stream).await?;
	let tmp_file = bytes_to_path(&tmp_file).ok_or_else(|| {
		io::Error::new(
			io::ErrorKind::Other,
			"prepare pvf recv_request: non utf-8 artifact path".to_string(),
		)
	})?;
	Ok((pvf, tmp_file))
}

async fn send_response(stream: &mut UnixStream, result: PrepareResult) -> io::Result<()> {
	framed_send(stream, &result.encode()).await
}

/// The entrypoint that the spawned prepare worker should start with. The `socket_path` specifies
/// the path to the socket used to communicate with the host. The `node_version`, if `Some`,
/// is checked against the worker version. A mismatch results in immediate worker termination.
/// `None` is used for tests and in other situations when version check is not necessary.
///
/// # Flow
///
/// This runs the following in a loop:
///
/// 1. Get the code and parameters for preparation from the host.
///
/// 2. Start a memory tracker in a separate thread.
///
/// 3. Start the CPU time monitor loop and the actual preparation in two separate threads.
///
/// 4. Select on the two threads created in step 3. If the CPU timeout was hit, the CPU time monitor
///    thread will trigger first.
///
/// 5. Stop the memory tracker and get the stats.
///
/// 6. If compilation succeeded, write the compiled artifact into a temporary file.
///
/// 7. Send the result of preparation back to the host. If any error occurred in the above steps, we
///    send that in the `PrepareResult`.
pub fn worker_entrypoint(socket_path: &str, node_version: Option<&str>) {
	worker_event_loop("prepare", socket_path, node_version, |mut stream| async move {
		let worker_pid = std::process::id();

		loop {
			let (pvf, dest) = recv_request(&mut stream).await?;
			gum::debug!(
				target: LOG_TARGET,
				%worker_pid,
				"worker: preparing artifact",
			);

			let cpu_time_start = ProcessTime::now();
			let preparation_timeout = pvf.prep_timeout();

			// Run the memory tracker.
			#[cfg(any(target_os = "linux", feature = "jemalloc-allocator"))]
			let (memory_tracker_tx, memory_tracker_rx) = channel::<()>();
			#[cfg(any(target_os = "linux", feature = "jemalloc-allocator"))]
			let memory_tracker_thread = thread::spawn(move || memory_tracker_loop(memory_tracker_rx));

			// Spawn a new thread that runs the CPU time monitor.
			let (cpu_time_monitor_tx, cpu_time_monitor_rx) = channel::<()>();
			let cpu_time_monitor_thread = thread::spawn(move || {
				cpu_time_monitor_loop(cpu_time_start, preparation_timeout, cpu_time_monitor_rx)
			});
			// Spawn another thread for preparation.
			let prepare_thread = thread::spawn(move || {
				let result = prepare_artifact(pvf);

				// Get the `ru_maxrss` stat. If supported, call getrusage for the thread.
				#[cfg(target_os = "linux")]
				let result = result.map(|artifact| (artifact, get_max_rss_thread()));

				result
			});

			// "Select" the first thread that finishes by checking all threads in a naive loop.
			let result = loop {
				// NOTE: The order in which we poll threads is important! This loop sleeps between
				// polling, so it is possible to go over the timeout even when the worker thread
				// finishes on time. So we want to prioritize selecting the worker thread and not
				// the CPU thread. If the measured CPU time is over the timeout, we will reject the
				// candidate later on the host-side. This avoids a source of indeterminism, where a
				// job can trigger timeouts on some validators and not others.
				if prepare_thread.is_finished() {
					let cpu_time_elapsed = cpu_time_start.elapsed();
					let _ = cpu_time_monitor_tx.send(());

					break match prepare_thread.join().unwrap_or_else(|err| {
						Err(PrepareError::Panic(stringify_panic_payload(err)))
					}) {
						Err(err) => {
							// Serialized error will be written into the socket.
							Err(err)
						},
						Ok(ok) => {
							// Stop the memory stats worker and get its observed memory stats.
							#[cfg(any(target_os = "linux", feature = "jemalloc-allocator"))]
							let memory_tracker_stats = get_memory_tracker_loop_stats(
								memory_tracker_thread,
								memory_tracker_tx,
								worker_pid,
							)
							.await;
							#[cfg(target_os = "linux")]
							let (ok, max_rss) = ok;
							let memory_stats = MemoryStats {
								#[cfg(any(target_os = "linux", feature = "jemalloc-allocator"))]
								memory_tracker_stats,
								#[cfg(target_os = "linux")]
								max_rss: extract_max_rss_stat(max_rss, worker_pid),
							};

							// Write the serialized artifact into a temp file.
							//
							// PVF host only keeps artifacts statuses in its memory, successfully
							// compiled code gets stored on the disk (and consequently deserialized
							// by execute-workers). The prepare worker is only required to send `Ok`
							// to the pool to indicate the success.

							gum::debug!(
								target: LOG_TARGET,
								%worker_pid,
								"worker: writing artifact to {}",
								dest.display(),
							);
							tokio::fs::write(&dest, &ok).await?;

							Ok(PrepareStats { cpu_time_elapsed, memory_stats })
						},
					}
				}
				// If this thread is not selected, the join handle is dropped and the thread will
				// finish in the background.
				if cpu_time_monitor_thread.is_finished() {
					break match cpu_time_monitor_thread.join() {
						Ok(Some(cpu_time_elapsed)) => {
							// Log if we exceed the timeout and the other thread hasn't finished.
							gum::warn!(
								target: LOG_TARGET,
								%worker_pid,
								"prepare job took {}ms cpu time, exceeded prepare timeout {}ms",
								cpu_time_elapsed.as_millis(),
								preparation_timeout.as_millis(),
							);
							Err(PrepareError::TimedOut)
						},
						Ok(None) => Err(PrepareError::IoErr(
							"error communicating over finished channel".into(),
						)),
						// Errors in this thread are independent of the candidate.
						Err(err) => Err(PrepareError::IoErr(stringify_panic_payload(err))),
					}
				}

				thread::sleep(Duration::from_millis(10));
			};

			send_response(&mut stream, result).await?;
		}
	});
}

fn prepare_artifact(pvf: PvfPrepData) -> Result<CompiledArtifact, PrepareError> {
	// TODO: Is this necessary? Panics are already caught by `std::thread::join`.
	panic::catch_unwind(|| {
		let blob = match prevalidate(&pvf.code()) {
			Err(err) => return Err(PrepareError::Prevalidation(format!("{:?}", err))),
			Ok(b) => b,
		};

		match prepare(blob, &pvf.executor_params()) {
			Ok(compiled_artifact) => Ok(CompiledArtifact::new(compiled_artifact)),
			Err(err) => Err(PrepareError::Preparation(format!("{:?}", err))),
		}
	})
	.map_err(|panic_payload| PrepareError::Panic(stringify_panic_payload(panic_payload)))
	.and_then(|inner_result| inner_result)
}
