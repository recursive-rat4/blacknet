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

use crate::circuit::builder::{CircuitBuilder, LinearCombination};
use crate::circuit::duplex::DuplexImpl;
use crate::circuit::jive::Jive;
use crate::circuit::permutation::Permutation;
use crate::circuit::poseidon2::Poseidon2Circuit;
use crate::pervushin::PervushinField;
use crate::poseidon2pervushin::{Poseidon2Pervushin8, Poseidon2Pervushin12};

impl Permutation<PervushinField> for Poseidon2Pervushin12 {
    type Domain = [LinearCombination<PervushinField>; 12];

    #[inline]
    fn permute(circuit: &CircuitBuilder<PervushinField>, x: &mut Self::Domain) {
        <Self as Poseidon2Circuit<PervushinField, 12, 48, 12, 48>>::permute(circuit, x)
    }
}

pub type DuplexPoseidon2Pervushin<'a, 'b> =
    DuplexImpl<'a, 'b, PervushinField, 8, 4, 12, Poseidon2Pervushin12>;

impl Permutation<PervushinField> for Poseidon2Pervushin8 {
    type Domain = [LinearCombination<PervushinField>; 8];

    #[inline]
    fn permute(circuit: &CircuitBuilder<PervushinField>, x: &mut Self::Domain) {
        <Self as Poseidon2Circuit<PervushinField, 8, 32, 12, 32>>::permute(circuit, x)
    }
}

pub type JivePoseidon2Pervushin<'a, 'b> = Jive<'a, 'b, PervushinField, 4, 8, Poseidon2Pervushin8>;
