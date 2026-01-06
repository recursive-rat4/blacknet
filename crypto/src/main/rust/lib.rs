/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod ajtaicommitment;
pub mod algebra;
pub mod assigner;
pub mod bigint;
pub mod chacha;
pub mod circuit;
pub mod compressionfunction;
pub mod constraintsystem;
pub mod convolution;
pub mod customizableconstraintsystem;
pub mod duplex;
pub mod ed25519;
pub mod fermat;
pub mod float;
pub mod integer;
pub mod jive;
pub mod johnsonlindenstrauss;
pub mod latticegadget;
pub mod lm;
pub mod matrix;
pub mod merkletree;
pub mod norm;
pub mod numbertheoretictransform;
pub mod permutation;
pub mod pervushin;
pub mod polynomial;
pub mod poseidon2;
pub mod poseidon2lm;
pub mod poseidon2pervushin;
pub mod r1cs;
pub mod random;
pub mod sumcheck;
pub mod twiddles;
pub mod z2;
