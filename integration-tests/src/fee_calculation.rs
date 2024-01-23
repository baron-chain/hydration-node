#![cfg(test)]

use crate::{oracle::hydradx_run_to_block, polkadot_test_net::*};
use frame_support::assert_ok;
use frame_support::dispatch::DispatchClass;
use frame_support::dispatch::GetDispatchInfo;
use hydradx_runtime::TransactionPayment;
use primitives::constants::currency::UNITS;
use primitives::constants::time::HOURS;
use primitives::{AssetId, Balance};
use sp_runtime::{FixedU128, Permill};
use xcm_emulator::TestExt;

use test_utils::assert_eq_approx;

const DOT_UNITS: u128 = 10_000_000_000;
const BTC_UNITS: u128 = 10_000_000;
const ETH_UNITS: u128 = 1_000_000_000_000_000_000;
const HDX_USD_SPOT_PRICE_IN_CENTS: Balance = 2; //1HDX =~ 2 CENTS;
const SWAP_ENCODED_LEN: u32 = 146; //We use this as this is what the UI send as length when omnipool swap is executed

#[test]
fn min_swap_fee() {
	TestNet::reset();
	Hydra::execute_with(|| {
		pallet_transaction_payment::pallet::NextFeeMultiplier::<hydradx_runtime::Runtime>::put(
			hydradx_runtime::MinimumMultiplier::get(),
		);

		let call = hydradx_runtime::RuntimeCall::Omnipool(pallet_omnipool::Call::<hydradx_runtime::Runtime>::sell {
			asset_in: DOT,
			asset_out: 2,
			amount: UNITS,
			min_buy_amount: 0,
		});

		let info = call.get_dispatch_info();
		let info_len = 146;
		let fee = TransactionPayment::compute_fee(info_len, &info, 0);
		let fee_in_cent = FixedU128::from(fee * HDX_USD_SPOT_PRICE_IN_CENTS).div(UNITS.into());
		let tolerance = FixedU128::from((2, (UNITS / 10_000)));
		println!("Swap tx fee in cents: {fee_in_cent:?}");

		assert_eq_approx!(
			fee_in_cent,
			FixedU128::from_float(1.040003910584000000),
			tolerance,
			"The min fee should be ~1 cent (0.01$)"
		);
	});
}

#[test]
fn max_swap_fee() {
	TestNet::reset();
	Hydra::execute_with(|| {
		pallet_transaction_payment::pallet::NextFeeMultiplier::<hydradx_runtime::Runtime>::put(
			hydradx_runtime::MaximumMultiplier::get(),
		);

		let call = hydradx_runtime::RuntimeCall::Omnipool(pallet_omnipool::Call::<hydradx_runtime::Runtime>::sell {
			asset_in: DOT,
			asset_out: 2,
			amount: UNITS,
			min_buy_amount: 0,
		});

		let info = call.get_dispatch_info();
		let info_len = 146; //We use this as this is what the UI send as length when omnipool swap is executed
		let fee = TransactionPayment::compute_fee(info_len, &info, 0);
		let fee_in_cent = FixedU128::from(fee * HDX_USD_SPOT_PRICE_IN_CENTS).div(UNITS.into());
		let tolerance = FixedU128::from((2, (UNITS / 10_000)));
		assert_eq_approx!(
			fee_in_cent,
			FixedU128::from_float(1002.399047518770000000),
			tolerance,
			"The max fee should be ~1000 cent (10$)"
		);
	});
}

#[test]
fn fee_growth_simulator_starting_with_genesis_chain() {
	TestNet::reset();

	Hydra::execute_with(|| {
		let prod_init_multiplier = FixedU128::from_inner(1000000000000000000);

		pallet_transaction_payment::pallet::NextFeeMultiplier::<hydradx_runtime::Runtime>::put(prod_init_multiplier);
		init_omnipool();
		init_oracle();
		let block_weight = hydradx_runtime::BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_total
			.unwrap();

		for b in 2..=HOURS {
			hydradx_run_to_block(b);
			hydradx_runtime::System::set_block_consumed_resources(block_weight, 0);
			let call =
				hydradx_runtime::RuntimeCall::Omnipool(pallet_omnipool::Call::<hydradx_runtime::Runtime>::sell {
					asset_in: HDX,
					asset_out: 2,
					amount: 10 * UNITS,
					min_buy_amount: 10000,
				});

			let info = call.get_dispatch_info();
			let fee = TransactionPayment::compute_fee(SWAP_ENCODED_LEN, &info, 0);
			let fee_in_cent = FixedU128::from(fee * HDX_USD_SPOT_PRICE_IN_CENTS).div(UNITS.into());

			let next = TransactionPayment::next_fee_multiplier();

			//let next = SlowAdjustingFeeUpdate::<Runtime>::convert(multiplier);
			println!("Swap tx fee in cents: {fee_in_cent:?} at block {b:?} with multiplier: {next:?}");
		}
	});
}

#[test]
fn fee_growth_simulator_with_idle_chain() {
	TestNet::reset();

	Hydra::execute_with(|| {
		//We simulate that the chain has no activity so the MinimumMultiplier kept diverged to absolute minimum
		pallet_transaction_payment::pallet::NextFeeMultiplier::<hydradx_runtime::Runtime>::put(
			hydradx_runtime::MinimumMultiplier::get(),
		);

		init_omnipool();
		init_oracle();
		let block_weight = hydradx_runtime::BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_total
			.unwrap();

		for b in 2..=HOURS {
			hydradx_run_to_block(b);
			hydradx_runtime::System::set_block_consumed_resources(block_weight, 0);
			let call =
				hydradx_runtime::RuntimeCall::Omnipool(pallet_omnipool::Call::<hydradx_runtime::Runtime>::sell {
					asset_in: HDX,
					asset_out: 2,
					amount: 10 * UNITS,
					min_buy_amount: 10000,
				});

			let info = call.get_dispatch_info();
			let fee = TransactionPayment::compute_fee(SWAP_ENCODED_LEN, &info, 0);
			let fee_in_cent = FixedU128::from(fee * HDX_USD_SPOT_PRICE_IN_CENTS).div(UNITS.into());

			let next = TransactionPayment::next_fee_multiplier();

			//let next = SlowAdjustingFeeUpdate::<Runtime>::convert(multiplier);
			println!("Swap tx fee in cents: {fee_in_cent:?} at block {b:?} with multiplier: {next:?}");
		}
	});
}

fn set_balance(who: hydradx_runtime::AccountId, currency: AssetId, amount: i128) {
	assert_ok!(hydradx_runtime::Currencies::update_balance(
		hydradx_runtime::RuntimeOrigin::root(),
		who,
		currency,
		amount,
	));
}

fn init_omnipool() {
	let native_price = FixedU128::from_inner(1201500000000000);
	let stable_price = FixedU128::from_inner(45_000_000_000);

	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::RuntimeOrigin::root(),
		HDX,
		native_price,
		Permill::from_percent(10),
		hydradx_runtime::Omnipool::protocol_account(),
	));
	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::RuntimeOrigin::root(),
		DAI,
		stable_price,
		Permill::from_percent(100),
		hydradx_runtime::Omnipool::protocol_account(),
	));

	let dot_price = FixedU128::from_inner(25_650_000_000_000_000_000);
	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::RuntimeOrigin::root(),
		DOT,
		dot_price,
		Permill::from_percent(100),
		AccountId::from(BOB),
	));

	let eth_price = FixedU128::from_inner(71_145_071_145_071);
	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::RuntimeOrigin::root(),
		ETH,
		eth_price,
		Permill::from_percent(100),
		AccountId::from(BOB),
	));

	let btc_price = FixedU128::from_inner(9_647_109_647_109_650_000_000_000);
	assert_ok!(hydradx_runtime::Omnipool::add_token(
		hydradx_runtime::RuntimeOrigin::root(),
		BTC,
		btc_price,
		Permill::from_percent(100),
		AccountId::from(BOB),
	));
	set_zero_reward_for_referrals(HDX);
	set_zero_reward_for_referrals(DAI);
	set_zero_reward_for_referrals(DOT);
	set_zero_reward_for_referrals(ETH);
}

/// This function executes one sell and buy with HDX for all assets in the omnipool. This is necessary to
/// oracle have a prices for the assets.
/// NOTE: It's necessary to change parachain block to oracle have prices.
fn init_oracle() {
	let trader = DAVE;

	set_balance(trader.into(), HDX, 1_000 * UNITS as i128);
	set_balance(trader.into(), DOT, 1_000 * DOT_UNITS as i128);
	set_balance(trader.into(), ETH, 1_000 * ETH_UNITS as i128);
	set_balance(trader.into(), BTC, 1_000 * BTC_UNITS as i128);

	assert_ok!(hydradx_runtime::Omnipool::sell(
		hydradx_runtime::RuntimeOrigin::signed(DAVE.into()),
		DOT,
		HDX,
		2 * DOT_UNITS,
		0,
	));

	assert_ok!(hydradx_runtime::Omnipool::buy(
		hydradx_runtime::RuntimeOrigin::signed(DAVE.into()),
		DOT,
		HDX,
		2 * DOT_UNITS,
		u128::MAX
	));

	assert_ok!(hydradx_runtime::Omnipool::sell(
		hydradx_runtime::RuntimeOrigin::signed(DAVE.into()),
		ETH,
		HDX,
		2 * ETH_UNITS,
		0,
	));

	assert_ok!(hydradx_runtime::Omnipool::buy(
		hydradx_runtime::RuntimeOrigin::signed(DAVE.into()),
		ETH,
		HDX,
		2 * ETH_UNITS,
		u128::MAX
	));

	assert_ok!(hydradx_runtime::Omnipool::sell(
		hydradx_runtime::RuntimeOrigin::signed(DAVE.into()),
		BTC,
		HDX,
		2 * BTC_UNITS,
		0,
	));

	assert_ok!(hydradx_runtime::Omnipool::buy(
		hydradx_runtime::RuntimeOrigin::signed(DAVE.into()),
		BTC,
		HDX,
		2 * BTC_UNITS,
		u128::MAX
	));
}
