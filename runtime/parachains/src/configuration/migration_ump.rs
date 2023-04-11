// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! A module that is responsible for migration of storage.

use crate::configuration::{self, ActiveConfig, Config, Pallet, LOG_TARGET};
use frame_support::{pallet_prelude::*, traits::StorageVersion, weights::Weight};
use frame_system::pallet_prelude::BlockNumberFor;
use primitives::vstaging::AsyncBackingParams;

pub use v5::MigrateV4ToV5;

/// The current storage version.
///
/// v0-v1: <https://github.com/paritytech/polkadot/pull/3575>
/// v1-v2: <https://github.com/paritytech/polkadot/pull/4420>
/// v2-v3: <https://github.com/paritytech/polkadot/pull/6091> (remove weights)
/// v3-v4: <https://github.com/paritytech/polkadot/pull/6345> (remove spam slots)
/// v4-v5: <https://github.com/paritytech/polkadot/pull/6271> (remove UMP dispatch queue)
pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);

pub mod v5 {
	use super::*;
	use frame_support::traits::OnRuntimeUpgrade;
	use primitives::{Balance, SessionIndex};
	#[cfg(feature = "try-runtime")]
	use sp_std::prelude::*;

	#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug)]
	pub struct V4HostConfiguration<BlockNumber> {
		pub max_code_size: u32,
		pub max_head_data_size: u32,
		pub max_upward_queue_count: u32,
		pub max_upward_queue_size: u32,
		pub max_upward_message_size: u32,
		pub max_upward_message_num_per_candidate: u32,
		pub hrmp_max_message_num_per_candidate: u32,
		pub validation_upgrade_cooldown: BlockNumber,
		pub validation_upgrade_delay: BlockNumber,
		pub max_pov_size: u32,
		pub max_downward_message_size: u32,
		pub ump_service_total_weight: Weight,
		pub hrmp_max_parachain_outbound_channels: u32,
		pub hrmp_max_parathread_outbound_channels: u32,
		pub hrmp_sender_deposit: Balance,
		pub hrmp_recipient_deposit: Balance,
		pub hrmp_channel_max_capacity: u32,
		pub hrmp_channel_max_total_size: u32,
		pub hrmp_max_parachain_inbound_channels: u32,
		pub hrmp_max_parathread_inbound_channels: u32,
		pub hrmp_channel_max_message_size: u32,
		pub code_retention_period: BlockNumber,
		pub parathread_cores: u32,
		pub parathread_retries: u32,
		pub group_rotation_frequency: BlockNumber,
		pub chain_availability_period: BlockNumber,
		pub thread_availability_period: BlockNumber,
		pub scheduling_lookahead: u32,
		pub max_validators_per_core: Option<u32>,
		pub max_validators: Option<u32>,
		pub dispute_period: SessionIndex,
		pub dispute_post_conclusion_acceptance_period: BlockNumber,
		pub dispute_conclusion_by_time_out_period: BlockNumber,
		pub no_show_slots: u32,
		pub n_delay_tranches: u32,
		pub zeroth_delay_tranche_width: u32,
		pub needed_approvals: u32,
		pub relay_vrf_modulo_samples: u32,
		pub ump_max_individual_weight: Weight,
		pub pvf_checking_enabled: bool,
		pub pvf_voting_ttl: SessionIndex,
		pub minimum_validation_upgrade_delay: BlockNumber,
	}

	impl<BlockNumber: Default + From<u32>> Default for V4HostConfiguration<BlockNumber> {
		fn default() -> Self {
			Self {
				group_rotation_frequency: 1u32.into(),
				chain_availability_period: 1u32.into(),
				thread_availability_period: 1u32.into(),
				no_show_slots: 1u32.into(),
				validation_upgrade_cooldown: Default::default(),
				validation_upgrade_delay: Default::default(),
				code_retention_period: Default::default(),
				max_code_size: Default::default(),
				max_pov_size: Default::default(),
				max_head_data_size: Default::default(),
				parathread_cores: Default::default(),
				parathread_retries: Default::default(),
				scheduling_lookahead: Default::default(),
				max_validators_per_core: Default::default(),
				max_validators: None,
				dispute_period: 6,
				dispute_post_conclusion_acceptance_period: 100.into(),
				dispute_conclusion_by_time_out_period: 200.into(),
				n_delay_tranches: Default::default(),
				zeroth_delay_tranche_width: Default::default(),
				needed_approvals: Default::default(),
				relay_vrf_modulo_samples: Default::default(),
				max_upward_queue_count: Default::default(),
				max_upward_queue_size: Default::default(),
				max_downward_message_size: Default::default(),
				ump_service_total_weight: Weight::zero(),
				max_upward_message_size: Default::default(),
				max_upward_message_num_per_candidate: Default::default(),
				hrmp_sender_deposit: Default::default(),
				hrmp_recipient_deposit: Default::default(),
				hrmp_channel_max_capacity: Default::default(),
				hrmp_channel_max_total_size: Default::default(),
				hrmp_max_parachain_inbound_channels: Default::default(),
				hrmp_max_parathread_inbound_channels: Default::default(),
				hrmp_channel_max_message_size: Default::default(),
				hrmp_max_parachain_outbound_channels: Default::default(),
				hrmp_max_parathread_outbound_channels: Default::default(),
				hrmp_max_message_num_per_candidate: Default::default(),
				ump_max_individual_weight: Weight::zero(),
				pvf_checking_enabled: false,
				pvf_voting_ttl: 2u32.into(),
				minimum_validation_upgrade_delay: 2.into(),
			}
		}
	}

	pub struct MigrateV4ToV5<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateV4ToV5<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			ensure!(StorageVersion::get::<Pallet<T>>() == 4, "The migration requires version 4");
			log::trace!(target: LOG_TARGET, "Going to upgraded parachain config...");

			Ok(Vec::new())
		}

		fn on_runtime_upgrade() -> Weight {
			if StorageVersion::get::<Pallet<T>>() == 4 {
				let weight_consumed = migrate_v4_to_v5::<T>();

				log::info!(target: LOG_TARGET, "MigrateV4ToV5 executed successfully");
				STORAGE_VERSION.put::<Pallet<T>>();

				weight_consumed
			} else {
				log::warn!(target: LOG_TARGET, "MigrateV4ToV5 should be removed.");
				T::DbWeight::get().reads(1)
			}
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
			log::trace!(target: LOG_TARGET, "Running post_upgrade()");
			ensure!(
				StorageVersion::get::<Pallet<T>>() == 5,
				"Storage version should be 5 after the migration"
			);

			Ok(())
		}
	}
}

fn migrate_v4_to_v5<T: Config>() -> Weight {
	// Unusual formatting is justified:
	// - make it easier to verify that fields assign what they supposed to assign.
	// - this code is transient and will be removed after all migrations are done.
	// - this code is important enough to optimize for legibility sacrificing consistency.
	#[rustfmt::skip]
	let translate =
		|pre: v5::V4HostConfiguration<BlockNumberFor<T>>| ->
configuration::HostConfiguration<BlockNumberFor<T>>
	{
		super::HostConfiguration {
max_code_size                            : pre.max_code_size,
max_head_data_size                       : pre.max_head_data_size,
max_upward_queue_count                   : pre.max_upward_queue_count,
max_upward_queue_size                    : pre.max_upward_queue_size,
max_upward_message_size                  : pre.max_upward_message_size,
max_upward_message_num_per_candidate     : pre.max_upward_message_num_per_candidate,
hrmp_max_message_num_per_candidate       : pre.hrmp_max_message_num_per_candidate,
validation_upgrade_cooldown              : pre.validation_upgrade_cooldown,
validation_upgrade_delay                 : pre.validation_upgrade_delay,
max_pov_size                             : pre.max_pov_size,
max_downward_message_size                : pre.max_downward_message_size,
hrmp_max_parachain_outbound_channels     : pre.hrmp_max_parachain_outbound_channels,
hrmp_max_parathread_outbound_channels    : pre.hrmp_max_parathread_outbound_channels,
hrmp_sender_deposit                      : pre.hrmp_sender_deposit,
hrmp_recipient_deposit                   : pre.hrmp_recipient_deposit,
hrmp_channel_max_capacity                : pre.hrmp_channel_max_capacity,
hrmp_channel_max_total_size              : pre.hrmp_channel_max_total_size,
hrmp_max_parachain_inbound_channels      : pre.hrmp_max_parachain_inbound_channels,
hrmp_max_parathread_inbound_channels     : pre.hrmp_max_parathread_inbound_channels,
hrmp_channel_max_message_size            : pre.hrmp_channel_max_message_size,
code_retention_period                    : pre.code_retention_period,
parathread_cores                         : pre.parathread_cores,
parathread_retries                       : pre.parathread_retries,
group_rotation_frequency                 : pre.group_rotation_frequency,
chain_availability_period                : pre.chain_availability_period,
thread_availability_period               : pre.thread_availability_period,
scheduling_lookahead                     : pre.scheduling_lookahead,
max_validators_per_core                  : pre.max_validators_per_core,
max_validators                           : pre.max_validators,
dispute_period                           : pre.dispute_period,
dispute_post_conclusion_acceptance_period: pre.dispute_post_conclusion_acceptance_period,
no_show_slots                            : pre.no_show_slots,
n_delay_tranches                         : pre.n_delay_tranches,
zeroth_delay_tranche_width               : pre.zeroth_delay_tranche_width,
needed_approvals                         : pre.needed_approvals,
relay_vrf_modulo_samples                 : pre.relay_vrf_modulo_samples,
pvf_checking_enabled                     : pre.pvf_checking_enabled,
pvf_voting_ttl                           : pre.pvf_voting_ttl,
minimum_validation_upgrade_delay         : pre.minimum_validation_upgrade_delay,

// Default values are zeroes, thus it's ensured allowed ancestry never crosses the upgrade block.
async_backing_params                     : AsyncBackingParams { max_candidate_depth: 0, allowed_ancestry_len: 0 },
		}
	};

	if let Err(err) = ActiveConfig::<T>::translate(|pre| pre.map(translate)) {
		// `Err` is returned when the pre-migration type cannot be deserialized. This
		// cannot happen if the migration runs correctly, i.e. against the expected version.
		//
		// This happening almost surely will lead to a panic somewhere else. Corruption seems
		// to be unlikely to be caused by this. So we just log. Maybe it'll work out still?
		log::error!(target: LOG_TARGET, "Could not translate ActiveConfig object: {:?}", err);
	}

	T::DbWeight::get().reads_writes(1, 1)
}

#[cfg(test)]
mod tests {
	use frame_support::weights::constants::{WEIGHT_PROOF_SIZE_PER_MB, WEIGHT_REF_TIME_PER_MILLIS};

	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::traits::OnRuntimeUpgrade;

	/// The old V4 config can be decoded from actual on-chain data.
	#[test]
	fn v4_deserialized_from_actual_data() {
		// Example how to get new `raw_config`:
		// We'll obtain the raw_config for block
		// 16,832,711 (0x2e3d78b87f0811f18a8b8469596aa7d8c9ef10d622b90d647f10ceebee3e380c) on Kusama.
		// Steps:
		// 1. Go to Polkadot.js -> Developer -> Chain state -> Storage: https://polkadot.js.org/apps/#/chainstate
		// 2. Set these parameters:
		//   2.1. selected state query: configuration; activeConfig(): PolkadotRuntimeParachainsConfigurationHostConfiguration
		//   2.2. blockhash to query at from above.
		//   2.3. Note the value of encoded storage key -> 0x06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385 for the referenced block.
		//   2.4. You'll also need the decoded values to update the test.
		// 3. Go to Polkadot.js -> Developer -> Chain state -> Raw storage
		//   3.1 Enter the encoded storage key and you get the raw config.

		// Fetched at Kusama 16,832,711 (0x2e3d78b87f0811f18a8b8469596aa7d8c9ef10d622b90d647f10ceebee3e380c)
		//
		// This exceeds the maximal line width length, but that's fine, since this is not code and
		// doesn't need to be read and also leaving it as one line allows to easily copy it.
		let raw_config = hex_literal::hex!["0000a000005000000a00000000c8000000c800000a0000000a000000100e0000580200000000500000c800000700e8764817020040011e00000000000000005039278c0400000000000000000000005039278c0400000000000000000000e8030000009001001e00000000000000009001008070000000000000000000000a0000000a0000000a00000001000000010500000001c80000000600000058020000580200000200000059000000000000001e000000280000000700c817a80402004001010200000014000000"];

		let v4 = v5::V4HostConfiguration::<primitives::BlockNumber>::decode(&mut &raw_config[..])
			.unwrap();

		// We check only a sample of the values here. If we missed any fields or messed up data types
		// that would skew all the fields coming after.
		assert_eq!(v4.max_code_size, 10_485_760);
		assert_eq!(v4.validation_upgrade_cooldown, 3600);
		assert_eq!(v4.max_pov_size, 5_242_880);
		assert_eq!(v4.hrmp_channel_max_message_size, 102_400);
		assert_eq!(v4.n_delay_tranches, 89);
		assert_eq!(
			v4.ump_max_individual_weight,
			Weight::from_parts(20 * WEIGHT_REF_TIME_PER_MILLIS, 5 * WEIGHT_PROOF_SIZE_PER_MB)
		);
		assert_eq!(v4.minimum_validation_upgrade_delay, 20);
	}

	#[test]
	fn test_migrate_v4_to_v5() {
		// Host configuration has lots of fields. However, in this migration we add only a couple of
		// fields. The most important part to check are a couple of the last fields. We also pick
		// extra fields to check arbitrarily, e.g. depending on their position (i.e. the middle) and
		// also their type.
		//
		// We specify only the picked fields and the rest should be provided by the `Default`
		// implementation. That implementation is copied over between the two types and should work
		// fine.
		let v4 = v5::V4HostConfiguration::<primitives::BlockNumber> {
			ump_max_individual_weight: Weight::from_parts(0x71616e6f6e0au64, 0x71616e6f6e0au64),
			needed_approvals: 69,
			thread_availability_period: 55,
			hrmp_recipient_deposit: 1337,
			max_pov_size: 1111,
			chain_availability_period: 33,
			minimum_validation_upgrade_delay: 20,
			..Default::default()
		};

		new_test_ext(Default::default()).execute_with(|| {
			// Implant the v4 version in the state.
			frame_support::storage::unhashed::put_raw(
				&configuration::ActiveConfig::<Test>::hashed_key(),
				&v4.encode(),
			);
			StorageVersion::new(4).put::<Pallet<Test>>();

			#[cfg(feature = "try-runtime")]
			MigrateV4ToV5::<Test>::pre_upgrade().unwrap();
			let _weight = MigrateV4ToV5::<Test>::on_runtime_upgrade();
			#[cfg(feature = "try-runtime")]
			MigrateV4ToV5::<Test>::post_upgrade(vec![]).unwrap();

			let v5 = configuration::ActiveConfig::<Test>::get();

			#[rustfmt::skip]
			{
				assert_eq!(v4.max_code_size                            , v5.max_code_size);
				assert_eq!(v4.max_head_data_size                       , v5.max_head_data_size);
				assert_eq!(v4.max_upward_queue_count                   , v5.max_upward_queue_count);
				assert_eq!(v4.max_upward_queue_size                    , v5.max_upward_queue_size);
				assert_eq!(v4.max_upward_message_size                  , v5.max_upward_message_size);
				assert_eq!(v4.max_upward_message_num_per_candidate     , v5.max_upward_message_num_per_candidate);
				assert_eq!(v4.hrmp_max_message_num_per_candidate       , v5.hrmp_max_message_num_per_candidate);
				assert_eq!(v4.validation_upgrade_cooldown              , v5.validation_upgrade_cooldown);
				assert_eq!(v4.validation_upgrade_delay                 , v5.validation_upgrade_delay);
				assert_eq!(v4.max_pov_size                             , v5.max_pov_size);
				assert_eq!(v4.max_downward_message_size                , v5.max_downward_message_size);
				assert_eq!(v4.hrmp_max_parachain_outbound_channels     , v5.hrmp_max_parachain_outbound_channels);
				assert_eq!(v4.hrmp_max_parathread_outbound_channels    , v5.hrmp_max_parathread_outbound_channels);
				assert_eq!(v4.hrmp_sender_deposit                      , v5.hrmp_sender_deposit);
				assert_eq!(v4.hrmp_recipient_deposit                   , v5.hrmp_recipient_deposit);
				assert_eq!(v4.hrmp_channel_max_capacity                , v5.hrmp_channel_max_capacity);
				assert_eq!(v4.hrmp_channel_max_total_size              , v5.hrmp_channel_max_total_size);
				assert_eq!(v4.hrmp_max_parachain_inbound_channels      , v5.hrmp_max_parachain_inbound_channels);
				assert_eq!(v4.hrmp_max_parathread_inbound_channels     , v5.hrmp_max_parathread_inbound_channels);
				assert_eq!(v4.hrmp_channel_max_message_size            , v5.hrmp_channel_max_message_size);
				assert_eq!(v4.code_retention_period                    , v5.code_retention_period);
				assert_eq!(v4.parathread_cores                         , v5.parathread_cores);
				assert_eq!(v4.parathread_retries                       , v5.parathread_retries);
				assert_eq!(v4.group_rotation_frequency                 , v5.group_rotation_frequency);
				assert_eq!(v4.chain_availability_period                , v5.chain_availability_period);
				assert_eq!(v4.thread_availability_period               , v5.thread_availability_period);
				assert_eq!(v4.scheduling_lookahead                     , v5.scheduling_lookahead);
				assert_eq!(v4.max_validators_per_core                  , v5.max_validators_per_core);
				assert_eq!(v4.max_validators                           , v5.max_validators);
				assert_eq!(v4.dispute_period                           , v5.dispute_period);
				assert_eq!(v4.dispute_post_conclusion_acceptance_period, v5.dispute_post_conclusion_acceptance_period);
				assert_eq!(v4.no_show_slots                            , v5.no_show_slots);
				assert_eq!(v4.n_delay_tranches                         , v5.n_delay_tranches);
				assert_eq!(v4.zeroth_delay_tranche_width               , v5.zeroth_delay_tranche_width);
				assert_eq!(v4.needed_approvals                         , v5.needed_approvals);
				assert_eq!(v4.relay_vrf_modulo_samples                 , v5.relay_vrf_modulo_samples);
				assert_eq!(v4.pvf_checking_enabled                     , v5.pvf_checking_enabled);
				assert_eq!(v4.pvf_voting_ttl                           , v5.pvf_voting_ttl);
				assert_eq!(v4.minimum_validation_upgrade_delay         , v5.minimum_validation_upgrade_delay);
			}; // ; makes this a statement. `rustfmt::skip` cannot be put on an expression.
		});
	}
}
