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

//! Autogenerated weights for pallet_referrals
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-12, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet_referrals
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// referrals.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bonds.
pub trait WeightInfo {
	fn register_code() -> Weight;
	fn link_code() -> Weight;
	fn convert() -> Weight;
	fn claim_rewards() -> Weight;
	fn set_reward_percentage() -> Weight;
}

/// Weights for pallet_referrals using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Referrals ReferralCodes (r:1 w:1)
	// Proof: Referrals ReferralCodes (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Referrals Referrer (r:0 w:1)
	// Proof: Referrals Referrer (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	fn register_code() -> Weight {
		// Minimum execution time: 61_000 nanoseconds.
		Weight::from_ref_time(62_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Referrals ReferralCodes (r:1 w:0)
	// Proof: Referrals ReferralCodes (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Referrals LinkedAccounts (r:1 w:1)
	// Proof: Referrals LinkedAccounts (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn link_code() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(27_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: DynamicFees AssetFee (r:1 w:0)
	// Proof: DynamicFees AssetFee (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:1 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: CircuitBreaker TradeVolumeLimitPerAsset (r:2 w:0)
	// Proof: CircuitBreaker TradeVolumeLimitPerAsset (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Referrals LinkedAccounts (r:1 w:0)
	// Proof: Referrals LinkedAccounts (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	// Storage: Staking Staking (r:1 w:0)
	// Proof: Staking Staking (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Referrals Assets (r:0 w:1)
	// Proof: Referrals Assets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	fn convert() -> Weight {
		// Minimum execution time: 304_000 nanoseconds.
		Weight::from_ref_time(307_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(21 as u64))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
	}
	// Storage: Referrals Assets (r:1 w:0)
	// Proof: Referrals Assets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	// Storage: Referrals Shares (r:1 w:1)
	// Proof: Referrals Shares (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Referrals TotalShares (r:1 w:1)
	// Proof: Referrals TotalShares (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Referrals Referrer (r:1 w:1)
	// Proof: Referrals Referrer (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	fn claim_rewards() -> Weight {
		// Minimum execution time: 73_000 nanoseconds.
		Weight::from_ref_time(74_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Referrals AssetTier (r:1 w:1)
	// Proof: Referrals AssetTier (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	fn set_reward_percentage() -> Weight {
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_ref_time(19_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

impl WeightInfo for () {
	// Storage: Referrals ReferralCodes (r:1 w:1)
	// Proof: Referrals ReferralCodes (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Referrals Referrer (r:0 w:1)
	// Proof: Referrals Referrer (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	fn register_code() -> Weight {
		// Minimum execution time: 61_000 nanoseconds.
		Weight::from_ref_time(62_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Referrals ReferralCodes (r:1 w:0)
	// Proof: Referrals ReferralCodes (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	// Storage: Referrals LinkedAccounts (r:1 w:1)
	// Proof: Referrals LinkedAccounts (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn link_code() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(27_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: DynamicFees AssetFee (r:1 w:0)
	// Proof: DynamicFees AssetFee (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:1 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: CircuitBreaker TradeVolumeLimitPerAsset (r:2 w:0)
	// Proof: CircuitBreaker TradeVolumeLimitPerAsset (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Referrals LinkedAccounts (r:1 w:0)
	// Proof: Referrals LinkedAccounts (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	// Storage: Staking Staking (r:1 w:0)
	// Proof: Staking Staking (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Referrals Assets (r:0 w:1)
	// Proof: Referrals Assets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	fn convert() -> Weight {
		// Minimum execution time: 304_000 nanoseconds.
		Weight::from_ref_time(307_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(21 as u64))
			.saturating_add(RocksDbWeight::get().writes(14 as u64))
	}
	// Storage: Referrals Assets (r:1 w:0)
	// Proof: Referrals Assets (max_values: None, max_size: Some(20), added: 2495, mode: MaxEncodedLen)
	// Storage: Referrals Shares (r:1 w:1)
	// Proof: Referrals Shares (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Referrals TotalShares (r:1 w:1)
	// Proof: Referrals TotalShares (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Referrals Referrer (r:1 w:1)
	// Proof: Referrals Referrer (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	fn claim_rewards() -> Weight {
		// Minimum execution time: 73_000 nanoseconds.
		Weight::from_ref_time(74_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Referrals AssetTier (r:1 w:1)
	// Proof: Referrals AssetTier (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	fn set_reward_percentage() -> Weight {
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_ref_time(19_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}