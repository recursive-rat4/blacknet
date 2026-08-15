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
use crate::circuit::builder::{CircuitBuilder, Constant, LinearCombination};
use crate::circuit::logicgate::LogicGate;
use crate::integer::Integer;
use crate::latticegadget::decompose_integer;
use crate::random::{Distribution, UniformGenerator};
use alloc::vec;
use alloc::vec::Vec;

pub struct BinaryUniformDistribution<'a, Z: IntegerModRing> {
    circuit: &'a CircuitBuilder<Z>,
    logic_gate: LogicGate<'a, Z>,
    cache: Vec<LinearCombination<Z>>,
    have_bits: u32,
}

impl<'a, Z: IntegerModRing + Clone> BinaryUniformDistribution<'a, Z> {
    pub fn new(circuit: &'a CircuitBuilder<Z>) -> Self {
        Self {
            circuit,
            logic_gate: LogicGate::new(circuit),
            cache: vec![LinearCombination::new(); Z::BITS as usize],
            have_bits: 0,
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

#[rustfmt::skip]
impl<
    'a,
    Z: IntegerModRing + Clone + Eq,
    G: UniformGenerator<Output = LinearCombination<Z>>
> Distribution<LinearCombination<Z>, G> for BinaryUniformDistribution<'a, Z>
where
    for<'b> &'b Z: RingOps<Z>,
{
    fn sample(&mut self, generator: &mut G) -> LinearCombination<Z> {
        if self.have_bits == 0 {
            let scope = self.circuit.scope("BinaryUniformDistribution::sample");
            let generated = generator.generate();
            let mut p = Z::ONE;
            let mut composed = LinearCombination::<Z>::new();
            for i in 0..Z::BITS {
                let digit = scope.auxiliary();
                self.cache[i as usize] = digit.into();
                composed += digit * Constant::<Z>::new(p.clone());
                p = p.double();
            }
            scope.constrain(composed, generated);
            let m1_gadget = Self::to_bits(&-Z::ONE);
            self.logic_gate.check_less_or_equal(&self.cache, &m1_gadget);
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
