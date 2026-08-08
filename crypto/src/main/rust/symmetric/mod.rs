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
mod trunc;

pub use blake2b::{Blake2b256, Blake2b512, Blake2bDuplexer, Blake2xb};
pub use chacha::ChaCha20;
pub use compressionfunction::CompressionFunction;
pub use duplex::{Absorb, Duplex, Duplexer, Phase, Squeeze, SqueezeWithSize, UniformDistribution};
pub use merkletree::MerkleTree;
pub use permutation::Permutation;
pub use trunc::Trunc;
