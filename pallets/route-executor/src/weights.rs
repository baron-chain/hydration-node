// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `pallet_route_executor`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-20, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dmoka-msi-pc`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=scripts/pallet-weight-template.hbs
// --pallet=pallet-route-executor
// --output=weights/router.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_route_executor`.
pub trait WeightInfo {
	fn skip_ed_handling_for_trade_with_insufficient_assets() -> Weight;
	fn calculate_and_execute_sell_in_lbp(c: u32, ) -> Weight;
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32, ) -> Weight;
	fn set_route_for_xyk() -> Weight;
	fn force_insert_route() -> Weight;
	fn get_oracle_price_for_xyk() -> Weight;
	fn get_oracle_price_for_omnipool() -> Weight;
	fn get_route() -> Weight;
	fn calculate_spot_price_with_fee_in_lbp() -> Weight;
}

/// Weights for `pallet_route_executor` using the HydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:0 w:1)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn skip_ed_handling_for_trade_with_insufficient_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `944`
		//  Estimated: `3590`
		// Minimum execution time: 13_600_000 picoseconds.
		Weight::from_parts(13_900_000, 3590)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1]`.
	fn calculate_and_execute_sell_in_lbp(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3743`
		//  Estimated: `13905`
		// Minimum execution time: 278_588_000 picoseconds.
		Weight::from_parts(284_606_595, 13905)
			// Standard Error: 341_686
			.saturating_add(Weight::from_parts(34_052_591, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(17_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `b` is `[0, 1]`.
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1637 + b * (2131 ±0)`
		//  Estimated: `6156 + b * (7749 ±0)`
		// Minimum execution time: 53_520_000 picoseconds.
		Weight::from_parts(54_339_000, 6156)
			// Standard Error: 418_984
			.saturating_add(Weight::from_parts(1_659_710, 0).saturating_mul(c.into()))
			// Standard Error: 919_791
			.saturating_add(Weight::from_parts(236_379_793, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((14_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7749).saturating_mul(b.into()))
	}
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Router::Routes` (r:1 w:1)
	/// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:7 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:15 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:6 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:7 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:5 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:5 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:0)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn set_route_for_xyk() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6426`
		//  Estimated: `39735`
		// Minimum execution time: 2_085_799_000 picoseconds.
		Weight::from_parts(2_111_539_000, 39735)
			.saturating_add(T::DbWeight::get().reads(56_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Router::Routes` (r:0 w:1)
	/// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn force_insert_route() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1012`
		//  Estimated: `0`
		// Minimum execution time: 23_300_000 picoseconds.
		Weight::from_parts(24_050_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_oracle_price_for_xyk() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1452`
		//  Estimated: `6294`
		// Minimum execution time: 26_531_000 picoseconds.
		Weight::from_parts(27_010_000, 6294)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: `EmaOracle::Oracles` (r:4 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_oracle_price_for_omnipool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1814`
		//  Estimated: `11598`
		// Minimum execution time: 40_726_000 picoseconds.
		Weight::from_parts(41_280_000, 11598)
			.saturating_add(T::DbWeight::get().reads(4_u64))
	}
	/// Storage: `Router::Routes` (r:1 w:0)
	/// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn get_route() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `800`
		//  Estimated: `3555`
		// Minimum execution time: 7_562_000 picoseconds.
		Weight::from_parts(7_748_000, 3555)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:0)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn calculate_spot_price_with_fee_in_lbp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1674`
		//  Estimated: `6156`
		// Minimum execution time: 36_086_000 picoseconds.
		Weight::from_parts(36_646_000, 6156)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:0 w:1)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn skip_ed_handling_for_trade_with_insufficient_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `944`
		//  Estimated: `3590`
		// Minimum execution time: 13_600_000 picoseconds.
		Weight::from_parts(13_900_000, 3590)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1]`.
	fn calculate_and_execute_sell_in_lbp(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3743`
		//  Estimated: `13905`
		// Minimum execution time: 278_588_000 picoseconds.
		Weight::from_parts(284_606_595, 13905)
			// Standard Error: 341_686
			.saturating_add(Weight::from_parts(34_052_591, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(17_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:1)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `b` is `[0, 1]`.
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1637 + b * (1923 ±0)`
		//  Estimated: `6156 + b * (7749 ±228_644_766_292_339_040)`
		// Minimum execution time: 94_614_000 picoseconds.
		Weight::from_parts(96_046_000, 6156)
			// Standard Error: 876_130
			.saturating_add(Weight::from_parts(3_658_596, 0).saturating_mul(c.into()))
			// Standard Error: 1_923_356
			.saturating_add(Weight::from_parts(272_645_114, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((14_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes((7_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7749).saturating_mul(b.into()))
	}
	/// Storage: `AssetRegistry::Assets` (r:6 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Router::Routes` (r:1 w:1)
	/// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:7 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:15 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:6 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Router::SkipEd` (r:1 w:0)
	/// Proof: `Router::SkipEd` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:7 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:5 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:5 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:0)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn set_route_for_xyk() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6426`
		//  Estimated: `39735`
		// Minimum execution time: 2_085_799_000 picoseconds.
		Weight::from_parts(2_111_539_000, 39735)
			.saturating_add(RocksDbWeight::get().reads(56_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Router::Routes` (r:0 w:1)
	/// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn force_insert_route() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1012`
		//  Estimated: `0`
		// Minimum execution time: 25_174_000 picoseconds.
		Weight::from_parts(25_689_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Router::Routes` (r:1 w:0)
	/// Proof: `Router::Routes` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn get_route() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `800`
		//  Estimated: `3555`
		// Minimum execution time: 7_562_000 picoseconds.
		Weight::from_parts(7_748_000, 3555)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}

	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_oracle_price_for_xyk() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1452`
		//  Estimated: `6294`
		// Minimum execution time: 21_820_000 picoseconds.
		Weight::from_parts(22_320_000, 6294)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: `EmaOracle::Oracles` (r:4 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_oracle_price_for_omnipool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1814`
		//  Estimated: `11598`
		// Minimum execution time: 35_720_000 picoseconds.
		Weight::from_parts(36_110_000, 11598)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
	}

	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Locks` (r:1 w:0)
	/// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn calculate_spot_price_with_fee_in_lbp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1674`
		//  Estimated: `6156`
		// Minimum execution time: 36_086_000 picoseconds.
		Weight::from_parts(36_646_000, 6156)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
	}
}
