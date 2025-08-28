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
use crate::assigner::distribution::Distribution;
use crate::assigner::logicgate::LogicGate;
use crate::distribution::UniformGenerator;
use crate::integer::Integer;
use crate::ring::{IntegerRing, UnitalRing};
use alloc::vec::Vec;

pub struct BinaryUniformDistribution<'a, G: UniformGenerator<Output: IntegerRing>> {
    cache: Vec<G::Output>,
    have_bits: u32,
    logic_gate: LogicGate<'a, G::Output>,
    assigment: &'a Assigment<G::Output>,
}

impl<'a, G: UniformGenerator<Output: IntegerRing>> BinaryUniformDistribution<'a, G> {
    fn useful_bits() -> u32 {
        if G::Output::MODULUS.count_ones() == 1 {
            G::Output::BITS
        } else {
            G::Output::BITS - 1
        }
    }
}

impl<'a, G: UniformGenerator<Output: IntegerRing>> Distribution<'a, G::Output, G>
    for BinaryUniformDistribution<'a, G>
{
    type Output = G::Output;

    fn new(assigment: &'a Assigment<G::Output>) -> Self {
        Self {
            cache: Vec::new(),
            have_bits: 0,
            logic_gate: LogicGate::new(assigment),
            assigment,
        }
    }

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        if self.have_bits == 0 {
            let gadget = generator.generate().gadget();
            self.assigment.extend_from_slice(&gadget);
            let m1_gadget = (-G::Output::UNITY).gadget(); //XXX make static?
            self.logic_gate.check_less_or_equal(&gadget, &m1_gadget);
            self.cache = gadget;
            self.have_bits = Self::useful_bits();
        }
        let result = self.cache[(Self::useful_bits() - self.have_bits) as usize];
        self.have_bits -= 1;
        result
    }
}
