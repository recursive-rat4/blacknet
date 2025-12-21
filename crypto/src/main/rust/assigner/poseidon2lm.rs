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

use crate::assigner::assigment::Assigment;
use crate::assigner::duplex::DuplexImpl;
use crate::assigner::jive::Jive;
use crate::assigner::permutation::Permutation;
use crate::assigner::poseidon2::Poseidon2Assigner;
use crate::lm::LMField;
use crate::poseidon2lm::{Poseidon2LM8, Poseidon2LM12};

impl Permutation<LMField> for Poseidon2LM12 {
    type Domain = [LMField; 12];

    #[inline]
    fn permute(assigment: &Assigment<LMField>, x: &mut Self::Domain) {
        <Self as Poseidon2Assigner<LMField, 12, 48, 26, 48>>::permute(assigment, x)
    }
}

pub type DuplexPoseidon2LM<'a> = DuplexImpl<'a, LMField, 8, 4, 12, Poseidon2LM12>;

impl Permutation<LMField> for Poseidon2LM8 {
    type Domain = [LMField; 8];

    #[inline]
    fn permute(assigment: &Assigment<LMField>, x: &mut Self::Domain) {
        <Self as Poseidon2Assigner<LMField, 8, 32, 26, 32>>::permute(assigment, x)
    }
}

pub type JivePoseidon2LM<'a> = Jive<'a, LMField, 4, 8, Poseidon2LM8>;
