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

#![allow(clippy::missing_safety_doc)]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod abeliangroup;
pub mod ajtaicommitment;
pub mod algebra;
pub mod assigner;
pub mod bigint;
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
pub mod eqextension;
pub mod fastdrg;
pub mod fermat;
pub mod field;
pub mod field25519;
pub mod float;
pub mod float01distribution;
pub mod group;
pub mod hypercube;
pub mod integer;
pub mod interpolation;
pub mod jive;
pub mod latticegadget;
pub mod lm;
pub mod magma;
pub mod matrixdense;
pub mod matrixring;
pub mod matrixsparse;
pub mod module;
pub mod monoid;
pub mod multilinearextension;
pub mod norm;
pub mod numbertheoretictransform;
pub mod permutation;
pub mod pervushin;
pub mod point;
pub mod polynomial;
pub mod polynomialringmonomial;
pub mod r1cs;
pub mod ring;
pub mod semigroup;
pub mod sumcheck;
pub mod twiddles;
pub mod twistededwardsgroup;
pub mod uniformintdistribution;
pub mod univariatepolynomial;
pub mod vectordense;
pub mod vectorsparse;
pub mod z2;
