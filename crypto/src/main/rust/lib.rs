/*
 * Copyright (c) 2025 Pavel Vasin
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

pub mod abeliangroup;
pub mod ajtaicommitment;
pub mod algebra;
pub mod assigner;
pub mod bigint;
pub mod binaritypolynomial;
pub mod binaryuniformdistribution;
pub mod chacha;
pub mod circuit;
pub mod compressionfunction;
pub mod constraintsystem;
pub mod convolution;
pub mod customizableconstraintsystem;
pub mod cyclicgroup;
pub mod distribution;
pub mod duplex;
pub mod edwards25519;
pub mod eqextension;
pub mod fastdrg;
pub mod fastrng;
pub mod fermat;
pub mod field;
pub mod field25519;
pub mod float;
pub mod float01distribution;
pub mod freemodule;
pub mod group;
pub mod hypercube;
pub mod integer;
pub mod interpolation;
pub mod jive;
pub mod latticegadget;
pub mod lm;
pub mod magma;
pub mod matrix;
pub mod matrixring;
pub mod module;
pub mod monoid;
pub mod multilinearextension;
pub mod norm;
pub mod nttring;
pub mod numbertheoretictransform;
pub mod operation;
pub mod permutation;
pub mod pervushin;
pub mod point;
pub mod polynomial;
pub mod poseidon2;
pub mod poseidon2lm;
pub mod poseidon2pervushin;
pub mod quartaryuniformdistribution;
pub mod r1cs;
pub mod ring;
pub mod semigroup;
pub mod semiring;
pub mod sumcheck;
pub mod twiddles;
pub mod twistededwardsgroup;
pub mod uniformintdistribution;
pub mod univariatepolynomial;
pub mod univariatering;
pub mod z2;
