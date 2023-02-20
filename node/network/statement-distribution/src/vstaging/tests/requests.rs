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

use super::*;

use polkadot_node_network_protocol::{self as net_protocol, vstaging as protocol_vstaging, PeerId};
use polkadot_node_primitives::{SignedFullStatement, Statement};
use polkadot_node_subsystem::{
	messages::{NetworkBridgeEvent, StatementDistributionMessage},
	overseer,
};
use polkadot_primitives::vstaging::{
	CompactStatement, Hash, HeadData, SignedStatement, SigningContext, UncheckedSignedStatement,
	ValidatorId, ValidatorIndex,
};
use polkadot_primitives_test_helpers::make_candidate;
use sc_keystore::LocalKeystore;
use sp_application_crypto::{sr25519::Pair, AppKey, Pair as TraitPair};
use sp_keyring::Sr25519Keyring;
use sp_keystore::{CryptoStore, SyncCryptoStorePtr};

use std::sync::Arc;

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
	let session_index = 1;

	test_harness(config, session_index, |test_state, mut virtual_overseer| async move {
		let relay_hash = Hash::repeat_byte(0x01);
		let (candidate, pvd) = make_candidate(
			relay_hash,
			1,
			1.into(),
			HeadData(vec![1, 2, 3]),
			HeadData(vec![1]),
			Hash::repeat_byte(42).into(),
		);
		let candidate_hash = candidate.hash();
		let leaf = TestLeaf {
			number: 1,
			hash: relay_hash,
			para_data: vec![(1.into(), PerParaData::new(97, HeadData(vec![1, 2, 3])))],
		};

		activate_leaf(&mut virtual_overseer, 1.into(), &leaf, &test_state).await;

		// virtual_overseer
		// 	.send(overseer::FromOrchestra::Communication {
		// 		msg: StatementDistributionMessage::Backed(candidate_hash),
		// 	})
		// 	.await;

		let peer_id = PeerId::random();
		let statement = {
			let signing_context = SigningContext { parent_hash: relay_hash, session_index };

			let keystore: SyncCryptoStorePtr = Arc::new(LocalKeystore::in_memory());
			let alice_public = CryptoStore::sr25519_generate_new(
				&*keystore,
				ValidatorId::ID,
				Some(&Sr25519Keyring::Alice.to_seed()),
			)
			.await
			.unwrap();

			SignedStatement::sign(
				&keystore,
				CompactStatement::Seconded(candidate_hash),
				&signing_context,
				ValidatorIndex(0),
				&alice_public.into(),
			)
			.await
			.ok()
			.flatten()
			.expect("should be signed")
		};

		// println!("{:?}", virtual_overseer.recv().await);

		// virtual_overseer
		// 	.send(overseer::FromOrchestra::Communication {
		// 		msg: StatementDistributionMessage::NetworkBridgeUpdate(
		// 			NetworkBridgeEvent::PeerMessage(
		// 				peer_id,
		// 				net_protocol::StatementDistributionMessage::VStaging(
		// 					protocol_vstaging::StatementDistributionMessage::Statement(
		// 						relay_hash,
		// 						statement.into_unchecked(),
		// 					),
		// 				),
		// 			),
		// 		),
		// 	})
		// 	.await;

		// println!("{:?}", virtual_overseer.recv().await);

		virtual_overseer
	});

	todo!()
}

// TODO [now]: peer reported for providing statements with invalid signatures or wrong validator IDs
