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

// TODO [now]: Remove once some tests are written.
#![allow(unused)]

use super::*;
use crate::*;
use polkadot_node_network_protocol::{
	request_response::{outgoing::Recipient, ReqProtocolNames},
	view, ObservedRole,
};
use polkadot_node_subsystem::messages::{
	network_bridge_event::NewGossipTopology, AllMessages, ChainApiMessage, FragmentTreeMembership,
	HypotheticalCandidate, NetworkBridgeEvent, ProspectiveParachainsMessage, RuntimeApiMessage,
	RuntimeApiRequest,
};
use polkadot_node_subsystem_test_helpers as test_helpers;
use polkadot_node_subsystem_types::{jaeger, ActivatedLeaf, LeafStatus};
use polkadot_primitives::vstaging::{
	AssignmentId, AssignmentPair, AsyncBackingParameters, BlockNumber, CandidateCommitments,
	CandidateDescriptor, CommittedCandidateReceipt, CoreState, GroupRotationInfo, HeadData, Header,
	IndexedVec, PersistedValidationData, ScheduledCore, SessionIndex, SessionInfo,
	ValidationCodeHash, ValidatorPair,
};
use sc_keystore::LocalKeystore;
use sp_application_crypto::Pair as PairT;
use sp_authority_discovery::AuthorityPair as AuthorityDiscoveryPair;
use sp_keyring::Sr25519Keyring;

use assert_matches::assert_matches;
use futures::Future;
use parity_scale_codec::Encode;
use rand::{Rng, SeedableRng};

use std::sync::Arc;

mod cluster;
mod grid;
mod requests;

type VirtualOverseer = test_helpers::TestSubsystemContextHandle<StatementDistributionMessage>;

const ASYNC_BACKING_PARAMETERS: AsyncBackingParameters =
	AsyncBackingParameters { max_candidate_depth: 4, allowed_ancestry_len: 3 };

// Some deterministic genesis hash for req/res protocol names
const GENESIS_HASH: Hash = Hash::repeat_byte(0xff);

struct TestConfig {
	validator_count: usize,
	// how many validators to place in each group.
	group_size: usize,
	// whether the local node should be a validator
	local_validator: bool,
}

#[derive(Clone)]
struct TestLocalValidator {
	validator_id: ValidatorId,
	validator_index: ValidatorIndex,
	group_index: GroupIndex,
}

struct TestState {
	config: TestConfig,
	local: Option<TestLocalValidator>,
	validators: Vec<ValidatorPair>,
	session_info: SessionInfo,
	req_sender: futures::channel::mpsc::Sender<sc_network::config::IncomingRequest>,
}

impl TestState {
	fn from_config(
		config: TestConfig,
		req_sender: futures::channel::mpsc::Sender<sc_network::config::IncomingRequest>,
		rng: &mut impl Rng,
	) -> Self {
		if config.group_size == 0 {
			panic!("group size cannot be 0");
		}

		let mut validators = Vec::new();
		let mut discovery_keys = Vec::new();
		let mut assignment_keys = Vec::new();
		let mut validator_groups = Vec::new();

		let local_validator_pos = if config.local_validator {
			// ensure local validator is always in a full group.
			Some(rng.gen_range(0..config.validator_count).saturating_sub(config.group_size - 1))
		} else {
			None
		};

		for i in 0..config.validator_count {
			let validator_pair = if Some(i) == local_validator_pos {
				// Note: the specific key is used to ensure the keystore holds
				// this key and the subsystem can detect that it is a validator.
				Sr25519Keyring::Ferdie.pair().into()
			} else {
				ValidatorPair::generate().0
			};
			let assignment_id = AssignmentPair::generate().0.public();
			let discovery_id = AuthorityDiscoveryPair::generate().0.public();

			let group_index = i / config.group_size;
			validators.push(validator_pair);
			discovery_keys.push(discovery_id);
			assignment_keys.push(assignment_id);
			if validator_groups.len() == group_index {
				validator_groups.push(vec![ValidatorIndex(i as _)]);
			} else {
				validator_groups.last_mut().unwrap().push(ValidatorIndex(i as _));
			}
		}

		let local = if let Some(local_pos) = local_validator_pos {
			Some(TestLocalValidator {
				validator_id: validators[local_pos].public().clone(),
				validator_index: ValidatorIndex(local_pos as _),
				group_index: GroupIndex((local_pos / config.group_size) as _),
			})
		} else {
			None
		};

		let validator_public = validator_pubkeys(&validators);
		let session_info = SessionInfo {
			validators: validator_public,
			discovery_keys,
			validator_groups: IndexedVec::from(validator_groups),
			assignment_keys,
			n_cores: 0,
			zeroth_delay_tranche_width: 0,
			relay_vrf_modulo_samples: 0,
			n_delay_tranches: 0,
			no_show_slots: 0,
			needed_approvals: 0,
			active_validator_indices: vec![],
			dispute_period: 6,
			random_seed: [0u8; 32],
		};

		TestState { config, local, validators, session_info, req_sender }
	}

	fn make_availability_cores(&self, f: impl Fn(usize) -> CoreState) -> Vec<CoreState> {
		(0..self.session_info.validator_groups.len()).map(f).collect()
	}

	fn group_validators(
		&self,
		group_index: GroupIndex,
		exclude_local: bool,
	) -> Vec<ValidatorIndex> {
		self.session_info
			.validator_groups
			.get(group_index)
			.unwrap()
			.iter()
			.cloned()
			.filter(|&i| {
				self.local.as_ref().map_or(true, |l| !exclude_local || l.validator_index != i)
			})
			.collect()
	}

	fn validator_id(&self, validator_index: ValidatorIndex) -> ValidatorId {
		self.session_info.validators.get(validator_index).unwrap().clone()
	}

	fn discovery_id(&self, validator_index: ValidatorIndex) -> AuthorityDiscoveryId {
		self.session_info.discovery_keys[validator_index.0 as usize].clone()
	}

	fn sign_statement(
		&self,
		validator_index: ValidatorIndex,
		statement: CompactStatement,
		context: &SigningContext,
	) -> SignedStatement {
		let payload = statement.signing_payload(context);
		let pair = &self.validators[validator_index.0 as usize];
		let signature = pair.sign(&payload[..]);

		SignedStatement::new(statement, validator_index, signature, context, &pair.public())
			.unwrap()
	}

	// send a request out, returning a future which expects a response.
	async fn send_request(
		&mut self,
		peer: PeerId,
		request: AttestedCandidateRequest,
	) -> impl Future<Output = sc_network::config::OutgoingResponse> {
		let (tx, rx) = futures::channel::oneshot::channel();
		let req = sc_network::config::IncomingRequest {
			peer,
			payload: request.encode(),
			pending_response: tx,
		};
		self.req_sender.send(req).await.unwrap();

		rx.map(|r| r.unwrap())
	}
}

fn test_harness<T: Future<Output = VirtualOverseer>>(
	config: TestConfig,
	test: impl FnOnce(TestState, VirtualOverseer) -> T,
) {
	let pool = sp_core::testing::TaskExecutor::new();
	let keystore = if config.local_validator {
		test_helpers::mock::make_ferdie_keystore()
	} else {
		Arc::new(LocalKeystore::in_memory()) as SyncCryptoStorePtr
	};
	let req_protocol_names = ReqProtocolNames::new(&GENESIS_HASH, None);
	let (statement_req_receiver, _) = IncomingRequest::get_config_receiver(&req_protocol_names);
	let (candidate_req_receiver, req_cfg) =
		IncomingRequest::get_config_receiver(&req_protocol_names);
	let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

	let test_state = TestState::from_config(config, req_cfg.inbound_queue.unwrap(), &mut rng);

	let (context, virtual_overseer) = test_helpers::make_subsystem_context(pool.clone());
	let subsystem = async move {
		let subsystem = crate::StatementDistributionSubsystem::new(
			keystore,
			statement_req_receiver,
			candidate_req_receiver,
			Metrics::default(),
			rng,
		);

		if let Err(e) = subsystem.run(context).await {
			panic!("Fatal error: {:?}", e);
		}
	};

	let test_fut = test(test_state, virtual_overseer);

	futures::pin_mut!(test_fut);
	futures::pin_mut!(subsystem);
	futures::executor::block_on(future::join(
		async move {
			let mut virtual_overseer = test_fut.await;
			virtual_overseer.send(FromOrchestra::Signal(OverseerSignal::Conclude)).await;
		},
		subsystem,
	));
}

struct PerParaData {
	min_relay_parent: BlockNumber,
	head_data: HeadData,
}

impl PerParaData {
	pub fn new(min_relay_parent: BlockNumber, head_data: HeadData) -> Self {
		Self { min_relay_parent, head_data }
	}
}

struct TestLeaf {
	number: BlockNumber,
	hash: Hash,
	parent_hash: Hash,
	session: SessionIndex,
	availability_cores: Vec<CoreState>,
	para_data: Vec<(ParaId, PerParaData)>,
}

impl TestLeaf {
	pub fn para_data(&self, para_id: ParaId) -> &PerParaData {
		self.para_data
			.iter()
			.find_map(|(p_id, data)| if *p_id == para_id { Some(data) } else { None })
			.unwrap()
	}
}

async fn activate_leaf(
	virtual_overseer: &mut VirtualOverseer,
	para_id: ParaId,
	leaf: &TestLeaf,
	test_state: &TestState,
	expect_session_info_request: bool,
) {
	let activated = ActivatedLeaf {
		hash: leaf.hash,
		number: leaf.number,
		status: LeafStatus::Fresh,
		span: Arc::new(jaeger::Span::Disabled),
	};

	virtual_overseer
		.send(FromOrchestra::Signal(OverseerSignal::ActiveLeaves(ActiveLeavesUpdate::start_work(
			activated,
		))))
		.await;

	handle_leaf_activation(
		virtual_overseer,
		para_id,
		leaf,
		test_state,
		expect_session_info_request,
	)
	.await;
}

async fn handle_leaf_activation(
	virtual_overseer: &mut VirtualOverseer,
	para_id: ParaId,
	leaf: &TestLeaf,
	test_state: &TestState,
	expect_session_info_request: bool,
) {
	let TestLeaf { number, hash, parent_hash, para_data, session, availability_cores } = leaf;
	let PerParaData { min_relay_parent, head_data } = leaf.para_data(para_id);

	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::RuntimeApi(
			RuntimeApiMessage::Request(parent, RuntimeApiRequest::StagingAsyncBackingParameters(tx))
		) if parent == *hash => {
			tx.send(Ok(ASYNC_BACKING_PARAMETERS)).unwrap();
		}
	);

	let mrp_response: Vec<(ParaId, BlockNumber)> = para_data
		.iter()
		.map(|(para_id, data)| (*para_id, data.min_relay_parent))
		.collect();
	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::ProspectiveParachains(
			ProspectiveParachainsMessage::GetMinimumRelayParents(parent, tx)
		) if parent == *hash => {
			tx.send(mrp_response).unwrap();
		}
	);

	let header = Header {
		parent_hash: *parent_hash,
		number: *number,
		state_root: Hash::zero(),
		extrinsics_root: Hash::zero(),
		digest: Default::default(),
	};
	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::ChainApi(
			ChainApiMessage::BlockHeader(parent, tx)
		) if parent == *hash => {
			tx.send(Ok(Some(header))).unwrap();
		}
	);

	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::RuntimeApi(
			RuntimeApiMessage::Request(parent, RuntimeApiRequest::SessionIndexForChild(tx))) if parent == *hash => {
			tx.send(Ok(*session)).unwrap();
		}
	);

	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::RuntimeApi(
			RuntimeApiMessage::Request(parent, RuntimeApiRequest::AvailabilityCores(tx))) if parent == *hash => {
			tx.send(Ok(availability_cores.clone())).unwrap();
		}
	);

	let validator_groups = test_state.session_info.validator_groups.to_vec();
	let group_rotation_info =
		GroupRotationInfo { session_start_block: 1, group_rotation_frequency: 12, now: 1 };
	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::RuntimeApi(
			RuntimeApiMessage::Request(parent, RuntimeApiRequest::ValidatorGroups(tx))) if parent == *hash => {
			tx.send(Ok((validator_groups, group_rotation_info))).unwrap();
		}
	);

	if expect_session_info_request {
		assert_matches!(
			virtual_overseer.recv().await,
			AllMessages::RuntimeApi(
				RuntimeApiMessage::Request(parent, RuntimeApiRequest::SessionInfo(s, tx))) if s == *session => {
				tx.send(Ok(Some(test_state.session_info.clone()))).unwrap();
			}
		);
	}
}

async fn answer_expected_hypothetical_depth_request(
	virtual_overseer: &mut VirtualOverseer,
	responses: Vec<(HypotheticalCandidate, FragmentTreeMembership)>,
	expected_leaf_hash: Option<Hash>,
	expected_backed_in_path_only: bool,
) {
	assert_matches!(
		virtual_overseer.recv().await,
		AllMessages::ProspectiveParachains(
			ProspectiveParachainsMessage::GetHypotheticalFrontier(req, tx)
		) => {
			assert_eq!(req.fragment_tree_relay_parent, expected_leaf_hash);
			assert_eq!(req.backed_in_path_only, expected_backed_in_path_only);
			for (i, (candidate, _)) in responses.iter().enumerate() {
				assert!(
					req.candidates.iter().find(|c| c == &candidate).is_some(),
					"did not receive request for hypothetical candidate {}",
					i,
				);
			}

			tx.send(responses);
		}
	)
}

fn validator_pubkeys(val_ids: &[ValidatorPair]) -> IndexedVec<ValidatorIndex, ValidatorId> {
	val_ids.iter().map(|v| v.public().into()).collect()
}

async fn connect_peer(
	virtual_overseer: &mut VirtualOverseer,
	peer: PeerId,
	authority_ids: Option<HashSet<AuthorityDiscoveryId>>,
) {
	virtual_overseer
		.send(FromOrchestra::Communication {
			msg: StatementDistributionMessage::NetworkBridgeUpdate(
				NetworkBridgeEvent::PeerConnected(
					peer,
					ObservedRole::Authority,
					ValidationVersion::VStaging.into(),
					authority_ids,
				),
			),
		})
		.await;
}

async fn disconnect_peer(virtual_overseer: &mut VirtualOverseer, peer: PeerId) {
	virtual_overseer
		.send(FromOrchestra::Communication {
			msg: StatementDistributionMessage::NetworkBridgeUpdate(
				NetworkBridgeEvent::PeerDisconnected(peer),
			),
		})
		.await;
}

async fn send_peer_view_change(virtual_overseer: &mut VirtualOverseer, peer: PeerId, view: View) {
	virtual_overseer
		.send(FromOrchestra::Communication {
			msg: StatementDistributionMessage::NetworkBridgeUpdate(
				NetworkBridgeEvent::PeerViewChange(peer, view),
			),
		})
		.await;
}

async fn send_peer_message(
	virtual_overseer: &mut VirtualOverseer,
	peer: PeerId,
	message: protocol_vstaging::StatementDistributionMessage,
) {
	virtual_overseer
		.send(FromOrchestra::Communication {
			msg: StatementDistributionMessage::NetworkBridgeUpdate(
				NetworkBridgeEvent::PeerMessage(peer, Versioned::VStaging(message)),
			),
		})
		.await;
}

async fn send_new_topology(virtual_overseer: &mut VirtualOverseer, topology: NewGossipTopology) {
	virtual_overseer
		.send(FromOrchestra::Communication {
			msg: StatementDistributionMessage::NetworkBridgeUpdate(
				NetworkBridgeEvent::NewGossipTopology(topology),
			),
		})
		.await;
}