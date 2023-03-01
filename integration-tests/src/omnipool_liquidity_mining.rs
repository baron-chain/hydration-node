// This file is part of HydraDX-node.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(test)]
use crate::polkadot_test_net::*;

use frame_support::assert_ok;
use warehouse_liquidity_mining::{
	DepositData, GlobalFarmData, GlobalFarmId, Instance1, LoyaltyCurve, YieldFarmData, YieldFarmEntry,
};

use polkadot_primitives::v2::BlockNumber;
use primitives::AssetId;
use sp_runtime::{
	traits::{One, Zero},
	FixedPointNumber, FixedU128, Permill, Perquintill,
};
use xcm_emulator::TestExt;

use hydradx_runtime::{Balance, Origin};
use pretty_assertions::assert_eq;

#[macro_export]
macro_rules! assert_nft_owner {
	( $coll:expr, $item: expr, $acc:expr ) => {{
		assert_eq!(hydradx_runtime::Uniques::owner($coll, $item).unwrap(), $acc);
	}};
}

#[test]
fn create_global_farm_should_work_when_origin_is_root() {
	TestNet::reset();

	Hydra::execute_with(|| {
		let total_rewards: Balance = 1_000_000 * UNITS;
		let planned_yielding_periods: BlockNumber = 1_000_000;
		let blocks_per_period: BlockNumber = 10;
		let reward_currency = HDX;
		let owner = Treasury::account_id();
		let yield_per_period = Perquintill::from_parts(570_776_255_707);
		let min_deposit = 1_000;
		//3[LRNA] = 1[HDX]
		let lrna_price_adjustment = FixedU128::checked_from_rational(1, 3).unwrap();

		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::Origin::root(),
			owner.clone(),
			total_rewards,
			0,
		));

		set_relaychain_block_number(100);

		assert_ok!(hydradx_runtime::OmnipoolLiquidityMining::create_global_farm(
			hydradx_runtime::Origin::root(),
			total_rewards,
			planned_yielding_periods,
			blocks_per_period,
			reward_currency,
			owner.clone(),
			yield_per_period,
			min_deposit,
			lrna_price_adjustment
		));

		let farm_id = 1;
		let updated_at = 100 / blocks_per_period;
		assert_eq!(
			hydradx_runtime::OmnipoolWarehouseLM::global_farm(1).unwrap(),
			GlobalFarmData::new(
				farm_id,
				updated_at,
				reward_currency,
				yield_per_period,
				planned_yielding_periods,
				blocks_per_period,
				owner,
				LRNA,
				total_rewards / planned_yielding_periods as u128,
				min_deposit,
				lrna_price_adjustment,
			)
		);

		let g_farm_account = hydradx_runtime::OmnipoolWarehouseLM::farm_account_id(farm_id).unwrap();
		assert_eq!(hydradx_runtime::Balances::free_balance(&g_farm_account), total_rewards);
	});
}

#[test]
fn create_yield_farm_should_work_when_asset_is_in_omnipool() {
	TestNet::reset();

	Hydra::execute_with(|| {
		let global_farm_id = 1;
		let created_yield_farm_id = 2;
		let loyalty_curve = Some(LoyaltyCurve::default());
		let multiplier = FixedU128::one();

		init_omnipool();

		set_relaychain_block_number(100);
		create_global_farm();

		set_relaychain_block_number(200);
		assert_ok!(hydradx_runtime::OmnipoolLiquidityMining::create_yield_farm(
			Origin::signed(Treasury::account_id()),
			global_farm_id,
			BTC,
			multiplier,
			loyalty_curve.clone()
		));

		let updated_at = 20;
		let y_farm = warehouse_liquidity_mining::YieldFarm::<hydradx_runtime::Runtime, Instance1>::get((
			BTC,
			global_farm_id,
			created_yield_farm_id,
		))
		.unwrap();
		assert_eq!(
			y_farm,
			YieldFarmData::new(created_yield_farm_id, updated_at, loyalty_curve, multiplier)
		);
	});
}

#[test]
fn deposit_shares_should_work_when_yield_farm_exists() {
	TestNet::reset();

	Hydra::execute_with(|| {
		let global_farm_id = 1;
		let yield_farm_id = 2;

		//Arrange
		init_omnipool();

		set_relaychain_block_number(100);
		create_global_farm();

		set_relaychain_block_number(200);
		create_yield_farm(global_farm_id, BTC);

		set_relaychain_block_number(300);

		assert_ok!(hydradx_runtime::Currencies::update_balance(
			hydradx_runtime::Origin::root(),
			BOB.into(),
			BTC,
			10_000 * UNITS as i128,
		));

		let position_id = omnipool_add_liquidity(BOB.into(), BTC, 1_000 * UNITS);
		assert_nft_owner!(hydradx_runtime::OmnipoolCollectionId::get(), position_id, BOB.into());

		//Act
		set_relaychain_block_number(400);
		assert_ok!(hydradx_runtime::OmnipoolLiquidityMining::deposit_shares(
			Origin::signed(BOB.into()),
			global_farm_id,
			yield_farm_id,
			position_id
		));

		//Assert
		let deposit = hydradx_runtime::OmnipoolWarehouseLM::deposit(1).unwrap();
		let mut expected_deposit = DepositData::new(1_000_000_000_000_000, BTC);
		expected_deposit
			.add_yield_farm_entry(YieldFarmEntry::new(
				global_farm_id,
				yield_farm_id,
				9_647_109_647_109_650_000_000,
				FixedU128::zero(),
				40,
				0,
			))
			.unwrap();

		assert_eq!(deposit, expected_deposit);

		//assert LM deposit
		assert_nft_owner!(hydradx_runtime::OmnipoolCollectionId::get(), 1, BOB.into());
		//original position owner should be palelt account
		let lm_account = hydradx_runtime::OmnipoolLiquidityMining::account_id();
		assert_nft_owner!(hydradx_runtime::OmnipoolCollectionId::get(), position_id, lm_account);
	});
}

fn init_omnipool() {
	let native_price = FixedU128::from_inner(1201500000000000);
	let stable_price = FixedU128::from_inner(45_000_000_000);

	assert_ok!(hydradx_runtime::Omnipool::set_tvl_cap(
		hydradx_runtime::Origin::root(),
		u128::MAX,
	));

	assert_ok!(hydradx_runtime::Omnipool::initialize_pool(
		hydradx_runtime::Origin::root(),
		stable_price,
		native_price,
		Permill::from_percent(100),
		Permill::from_percent(10)
	));

	let token_price = FixedU128::from_inner(25_650_000_000_000_000_000);

	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::Origin::root(),
		DOT,
		token_price,
		Permill::from_percent(100),
		AccountId::from(BOB),
	));

	let token_price = FixedU128::from_inner(71_145_071_145_071);

	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::Origin::root(),
		ETH,
		token_price,
		Permill::from_percent(100),
		AccountId::from(BOB),
	));

	let btc_price = FixedU128::from_inner(9_647_109_647_109_650_000_000_000);

	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::Origin::root(),
		BTC,
		btc_price,
		Permill::from_percent(100),
		AccountId::from(BOB),
	));
}

fn create_global_farm() {
	let total_rewards = 1_000_000 * UNITS;

	assert_ok!(hydradx_runtime::Balances::set_balance(
		hydradx_runtime::Origin::root(),
		Treasury::account_id(),
		total_rewards,
		0,
	));

	assert_ok!(hydradx_runtime::OmnipoolLiquidityMining::create_global_farm(
		hydradx_runtime::Origin::root(),
		total_rewards,
		1_000_000,
		10,
		HDX,
		Treasury::account_id(),
		Perquintill::from_parts(570_776_255_707),
		1_000,
		FixedU128::checked_from_rational(1, 3).unwrap()
	));
}

fn create_yield_farm(id: GlobalFarmId, asset: AssetId) {
	assert_ok!(hydradx_runtime::OmnipoolLiquidityMining::create_yield_farm(
		Origin::signed(Treasury::account_id()),
		id,
		asset,
		FixedU128::one(),
		Some(LoyaltyCurve::default())
	));
}

fn omnipool_add_liquidity(lp: AccountId, asset: AssetId, amount: Balance) -> primitives::ItemId {
	use hydradx_runtime::Omnipool;

	let current_position_id = Omnipool::next_position_id();

	assert_ok!(Omnipool::add_liquidity(Origin::signed(lp), asset, amount));

	current_position_id
}
