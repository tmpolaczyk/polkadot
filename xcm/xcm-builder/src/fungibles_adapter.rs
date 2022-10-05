// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! Adapters to work with `frame_support::traits::tokens::fungibles` through XCM.

use frame_support::traits::{tokens::fungibles, Contains, Get};
use sp_std::{marker::PhantomData, prelude::*, result};
use xcm::latest::prelude::*;
use xcm_executor::traits::{Convert, Error as MatchError, MatchesFungibles, TransactAsset};

pub struct FungiblesTransferAdapter<Assets, Matcher, AccountIdConverter, AccountId>(
	PhantomData<(Assets, Matcher, AccountIdConverter, AccountId)>,
);
impl<
		Assets: fungibles::Transfer<AccountId>,
		Matcher: MatchesFungibles<Assets::AssetId, Assets::Balance>,
		AccountIdConverter: Convert<MultiLocation, AccountId>,
		AccountId: Clone, // can't get away without it since Currency is generic over it.
	> TransactAsset for FungiblesTransferAdapter<Assets, Matcher, AccountIdConverter, AccountId>
{
	fn internal_transfer_asset(
		what: &MultiAsset,
		from: &MultiLocation,
		to: &MultiLocation,
		_context: &XcmContext,
	) -> result::Result<xcm_executor::Assets, XcmError> {
		log::trace!(
			target: "xcm::fungibles_adapter",
			"internal_transfer_asset what: {:?}, from: {:?}, to: {:?}",
			what, from, to
		);
		// Check we handle this asset.
		let (asset_id, amount) = Matcher::matches_fungibles(what)?;
		let source = AccountIdConverter::convert_ref(from)
			.map_err(|()| MatchError::AccountIdConversionFailed)?;
		let dest = AccountIdConverter::convert_ref(to)
			.map_err(|()| MatchError::AccountIdConversionFailed)?;
		Assets::transfer(asset_id, &source, &dest, amount, true)
			.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
		Ok(what.clone().into())
	}
}

pub struct FungiblesMutateAdapter<
	Assets,
	Matcher,
	AccountIdConverter,
	AccountId,
	CheckAsset,
	CheckingAccount,
>(PhantomData<(Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount)>);
impl<
		Assets: fungibles::Mutate<AccountId>,
		Matcher: MatchesFungibles<Assets::AssetId, Assets::Balance>,
		AccountIdConverter: Convert<MultiLocation, AccountId>,
		AccountId: Clone, // can't get away without it since Currency is generic over it.
		CheckAsset: Contains<Assets::AssetId>,
		CheckingAccount: Get<AccountId>,
	> TransactAsset
	for FungiblesMutateAdapter<
		Assets,
		Matcher,
		AccountIdConverter,
		AccountId,
		CheckAsset,
		CheckingAccount,
	>
{
	fn can_check_in(
		_origin: &MultiLocation,
		what: &MultiAsset,
		_context: &XcmContext,
	) -> XcmResult {
		log::trace!(
			target: "xcm::fungibles_adapter",
			"can_check_in origin: {:?}, what: {:?}",
			_origin, what
		);
		// Check we handle this asset.
		let (asset_id, amount) = Matcher::matches_fungibles(what)?;
		if CheckAsset::contains(&asset_id) {
			// This is an asset whose teleports we track.
			let checking_account = CheckingAccount::get();
			Assets::can_withdraw(asset_id, &checking_account, amount)
				.into_result()
				.map_err(|_| XcmError::NotWithdrawable)?;
		}
		Ok(())
	}

	fn check_in(_origin: &MultiLocation, what: &MultiAsset, _context: &XcmContext) {
		log::trace!(
			target: "xcm::fungibles_adapter",
			"check_in origin: {:?}, what: {:?}",
			_origin, what
		);
		if let Ok((asset_id, amount)) = Matcher::matches_fungibles(what) {
			if CheckAsset::contains(&asset_id) {
				let checking_account = CheckingAccount::get();
				let ok = Assets::burn_from(asset_id, &checking_account, amount).is_ok();
				debug_assert!(
					ok,
					"`can_check_in` must have returned `true` immediately prior; qed"
				);
			}
		}
	}

	fn check_out(_dest: &MultiLocation, what: &MultiAsset, _context: &XcmContext) {
		log::trace!(
			target: "xcm::fungibles_adapter",
			"check_out dest: {:?}, what: {:?}",
			_dest, what
		);
		if let Ok((asset_id, amount)) = Matcher::matches_fungibles(what) {
			if CheckAsset::contains(&asset_id) {
				let checking_account = CheckingAccount::get();
				let ok = Assets::mint_into(asset_id, &checking_account, amount).is_ok();
				debug_assert!(ok, "`mint_into` cannot generally fail; qed");
			}
		}
	}

	fn deposit_asset(what: &MultiAsset, who: &MultiLocation, _context: &XcmContext) -> XcmResult {
		log::trace!(
			target: "xcm::fungibles_adapter",
			"deposit_asset what: {:?}, who: {:?}",
			what, who,
		);
		// Check we handle this asset.
		let (asset_id, amount) = Matcher::matches_fungibles(what)?;
		let who = AccountIdConverter::convert_ref(who)
			.map_err(|()| MatchError::AccountIdConversionFailed)?;
		Assets::mint_into(asset_id, &who, amount)
			.map_err(|e| XcmError::FailedToTransactAsset(e.into()))
	}

	fn withdraw_asset(
		what: &MultiAsset,
		who: &MultiLocation,
		_maybe_context: Option<&XcmContext>,
	) -> result::Result<xcm_executor::Assets, XcmError> {
		log::trace!(
			target: "xcm::fungibles_adapter",
			"withdraw_asset what: {:?}, who: {:?}",
			what, who,
		);
		// Check we handle this asset.
		let (asset_id, amount) = Matcher::matches_fungibles(what)?;
		let who = AccountIdConverter::convert_ref(who)
			.map_err(|()| MatchError::AccountIdConversionFailed)?;
		Assets::burn_from(asset_id, &who, amount)
			.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
		Ok(what.clone().into())
	}
}

pub struct FungiblesAdapter<
	Assets,
	Matcher,
	AccountIdConverter,
	AccountId,
	CheckAsset,
	CheckingAccount,
>(PhantomData<(Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount)>);
impl<
		Assets: fungibles::Mutate<AccountId> + fungibles::Transfer<AccountId>,
		Matcher: MatchesFungibles<Assets::AssetId, Assets::Balance>,
		AccountIdConverter: Convert<MultiLocation, AccountId>,
		AccountId: Clone, // can't get away without it since Currency is generic over it.
		CheckAsset: Contains<Assets::AssetId>,
		CheckingAccount: Get<AccountId>,
	> TransactAsset
	for FungiblesAdapter<Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount>
{
	fn can_check_in(origin: &MultiLocation, what: &MultiAsset, context: &XcmContext) -> XcmResult {
		FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::can_check_in(origin, what, context)
	}

	fn check_in(origin: &MultiLocation, what: &MultiAsset, context: &XcmContext) {
		FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::check_in(origin, what, context)
	}

	fn check_out(dest: &MultiLocation, what: &MultiAsset, context: &XcmContext) {
		FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::check_out(dest, what, context)
	}

	fn deposit_asset(what: &MultiAsset, who: &MultiLocation, context: &XcmContext) -> XcmResult {
		FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::deposit_asset(what, who, context)
	}

	fn withdraw_asset(
		what: &MultiAsset,
		who: &MultiLocation,
		maybe_context: Option<&XcmContext>,
	) -> result::Result<xcm_executor::Assets, XcmError> {
		FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::withdraw_asset(what, who, maybe_context)
	}

	fn internal_transfer_asset(
		what: &MultiAsset,
		from: &MultiLocation,
		to: &MultiLocation,
		context: &XcmContext,
	) -> result::Result<xcm_executor::Assets, XcmError> {
		FungiblesTransferAdapter::<Assets, Matcher, AccountIdConverter, AccountId>::internal_transfer_asset(
			what, from, to, context
		)
	}
}