// Copyright 2023 Parity Technologies (UK) Ltd.
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

use super::{test_harness, TestConfig};

use polkadot_node_network_protocol::{self as net_protocol, vstaging as protocol_vstaging, PeerId};
use polkadot_node_subsystem::{
	messages::{NetworkBridgeEvent, StatementDistributionMessage},
	overseer,
};
use polkadot_primitives::vstaging::{CandidateHash, Hash, UncheckedSignedStatement};

// TODO [now]: peer reported for providing statements meant to be masked out

// TODO [now]: peer reported for not providing enough statements, request retried

#[test]
fn peer_reported_for_providing_duplicate_statements() {
	let config = TestConfig {
		validator_count: 50,
		group_size: 3,
		local_validator: true,
		// TODO: What is the idea for this? Perhaps there should be a constructor that seeds this
		// from e.g. time and prints it out (would only appear on test failure).
		rng_seed: 0,
	};

	test_harness(config, |state, mut virtual_overseer| async {
		let relay_parent = Hash::repeat_byte(0x01);
		let candidate_hash = CandidateHash(Hash::repeat_byte(0x11));

		virtual_overseer
			.send(overseer::FromOrchestra::Communication {
				msg: StatementDistributionMessage::Backed(candidate_hash),
			})
			.await;

		let statement = UncheckedSignedStatement::new();

		virtual_overseer
			.send(overseer::FromOrchestra::Communication {
				msg: StatementDistributionMessage::NetworkBridgeUpdate(
					NetworkBridgeEvent::PeerMessage(
						// TODO: How to get peer id?
						PeerId(state.local.unwrap().validator_id),
						net_protocol::StatementDistributionMessage::VStaging(
							protocol_vstaging::StatementDistributionMessage::Statement(
								relay_parent,
								statement,
							),
						),
					),
				),
			})
			.await;

		virtual_overseer
	});

	todo!()
}

// TODO [now]: peer reported for providing statements with invalid signatures or wrong validator IDs
