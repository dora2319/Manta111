// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_manta_sbt
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dolphin-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/manta
// benchmark
// pallet
// --chain=dolphin-dev
// --pallet=pallet_manta_sbt
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --output=./pallets/manta-sbt/src/weights.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_manta_sbt.
pub trait WeightInfo {
    fn to_private() -> Weight;
    fn reserve_sbt() -> Weight;
    fn change_allowlist_account() -> Weight;
    fn allowlist_evm_account() -> Weight;
    fn set_mint_chain_info() -> Weight;
    fn mint_sbt_eth() -> Weight;
}

/// Weights for pallet_manta_sbt using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_manta_sbt::WeightInfo for SubstrateWeight<T> {
    // Storage: MantaSbt ReservedIds (r:1 w:1)
    // Storage: MantaSbt UtxoSet (r:1 w:1)
    // Storage: MantaSbt ShardTrees (r:1 w:1)
    // Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaSbt SbtMetadata (r:0 w:1)
    // Storage: MantaSbt Shards (r:0 w:1)
    fn to_private() -> Weight {
        (29_910_952_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: MantaSbt NextSbtId (r:1 w:1)
    // Storage: MantaSbt ReservedIds (r:0 w:1)
    fn reserve_sbt() -> Weight {
        (47_080_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: MantaSbt AllowlistAccount (r:0 w:1)
    fn change_allowlist_account() -> Weight {
        (13_771_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: MantaSbt AllowlistAccount (r:1 w:0)
    // Storage: MantaSbt NextSbtId (r:1 w:1)
    // Storage: MantaSbt EvmAddressAllowlist (r:0 w:1)
    fn allowlist_evm_account() -> Weight {
        (20_203_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: MantaSbt MintChainInfos (r:0 w:1)
    fn set_mint_chain_info() -> Weight {
        (14_838_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: MantaSbt MintChainInfos (r:1 w:0)
    // Storage: System BlockHash (r:1 w:0)
    // Storage: MantaSbt EvmAddressAllowlist (r:1 w:1)
    // Storage: MantaSbt UtxoSet (r:1 w:1)
    // Storage: MantaSbt ShardTrees (r:1 w:1)
    // Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaSbt SbtMetadata (r:0 w:1)
    // Storage: MantaSbt Shards (r:0 w:1)
    fn mint_sbt_eth() -> Weight {
        (29_952_520_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: MantaSbt ReservedIds (r:1 w:1)
    // Storage: MantaSbt UtxoSet (r:1 w:1)
    // Storage: MantaSbt ShardTrees (r:1 w:1)
    // Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaSbt SbtMetadata (r:0 w:1)
    // Storage: MantaSbt Shards (r:0 w:1)
    fn to_private() -> Weight {
        (29_910_952_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: MantaSbt NextSbtId (r:1 w:1)
    // Storage: MantaSbt ReservedIds (r:0 w:1)
    fn reserve_sbt() -> Weight {
        (47_080_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: MantaSbt AllowlistAccount (r:0 w:1)
    fn change_allowlist_account() -> Weight {
        (13_771_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: MantaSbt AllowlistAccount (r:1 w:0)
    // Storage: MantaSbt NextSbtId (r:1 w:1)
    // Storage: MantaSbt EvmAddressAllowlist (r:0 w:1)
    fn allowlist_evm_account() -> Weight {
        (20_203_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: MantaSbt MintChainInfos (r:0 w:1)
    fn set_mint_chain_info() -> Weight {
        (14_838_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: MantaSbt MintChainInfos (r:1 w:0)
    // Storage: System BlockHash (r:1 w:0)
    // Storage: MantaSbt EvmAddressAllowlist (r:1 w:1)
    // Storage: MantaSbt UtxoSet (r:1 w:1)
    // Storage: MantaSbt ShardTrees (r:1 w:1)
    // Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
    // Storage: MantaSbt SbtMetadata (r:0 w:1)
    // Storage: MantaSbt Shards (r:0 w:1)
    fn mint_sbt_eth() -> Weight {
        (29_952_520_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
}