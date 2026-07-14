/*
 * Copyright (c) 2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! Symmetric cryptography.

pub mod blake2b;
pub mod chacha;
mod compressionfunction;
mod duplex;
mod merkletree;
mod permutation;
mod poseidon2;
mod poseidon2lm;
mod poseidon2pervushin;
mod trunc;

pub use blake2b::{Blake2b256, Blake2b512, Blake2xb};
pub use chacha::ChaCha20;
pub use compressionfunction::CompressionFunction;
pub use duplex::{Absorb, Duplex, Duplexer, Phase, Squeeze, SqueezeWithSize};
pub use merkletree::MerkleTree;
pub use permutation::Permutation;
pub use poseidon2::{Poseidon2Params, Poseidon2Plain};
pub use poseidon2lm::{DuplexPoseidon2LM, Poseidon2LM8, Poseidon2LM12, TruncPoseidon2LM};
pub use poseidon2pervushin::{
    DuplexPoseidon2Pervushin, Poseidon2Pervushin8, Poseidon2Pervushin12, TruncPoseidon2Pervushin,
};
pub use trunc::Trunc;
