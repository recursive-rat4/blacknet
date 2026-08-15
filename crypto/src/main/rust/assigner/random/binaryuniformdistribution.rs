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

use crate::algebra::{IntegerModRing, RingOps};
use crate::assigner::assigment::Assigment;
use crate::assigner::logicgate::LogicGate;
use crate::integer::Integer;
use crate::latticegadget::decompose_integer;
use crate::random::{Distribution, UniformGenerator};
use alloc::vec::Vec;

pub struct BinaryUniformDistribution<'a, Z: IntegerModRing> {
    cache: Vec<Z>,
    have_bits: u32,
    logic_gate: LogicGate<'a, Z>,
    assigment: &'a Assigment<Z>,
}

impl<'a, Z: IntegerModRing> BinaryUniformDistribution<'a, Z> {
    pub const fn new(assigment: &'a Assigment<Z>) -> Self {
        Self {
            cache: Vec::new(),
            have_bits: 0,
            logic_gate: LogicGate::new(assigment),
            assigment,
        }
    }

    pub const fn reset(&mut self) {
        self.have_bits = 0
    }

    fn useful_bits() -> u32 {
        if Z::MODULUS.count_ones() == 1 {
            Z::BITS
        } else {
            Z::BITS - 1
        }
    }

    fn to_bits(integer: &Z) -> Vec<Z> {
        decompose_integer(integer, Z::Int::LIMB_ONE, Z::Int::LIMB_ONE, Z::BITS).into()
    }
}

impl<'a, Z: IntegerModRing + Clone + Eq, G: UniformGenerator<Output = Z>> Distribution<Z, G>
    for BinaryUniformDistribution<'a, Z>
where
    for<'b> &'b Z: RingOps<Z>,
{
    fn sample(&mut self, generator: &mut G) -> Z {
        if self.have_bits == 0 {
            let gadget = Self::to_bits(&generator.generate());
            self.assigment.extend_from_slice(&gadget);
            let m1_gadget = Self::to_bits(&-Z::ONE); //XXX make static?
            self.logic_gate.check_less_or_equal(&gadget, &m1_gadget);
            self.cache = gadget;
            self.have_bits = Self::useful_bits();
        }
        let result = self.cache[(Self::useful_bits() - self.have_bits) as usize].clone();
        self.have_bits -= 1;
        result
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
