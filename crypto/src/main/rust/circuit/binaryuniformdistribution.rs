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

use crate::circuit::circuitbuilder::{CircuitBuilder, Constant, LinearCombination};
use crate::circuit::distribution::Distribution;
use crate::circuit::logicgate::LogicGate;
use crate::distribution::UniformGenerator;
use crate::integer::Integer;
use crate::ring::IntegerRing;

pub struct BinaryUniformDistribution<
    'a,
    Z: IntegerRing,
    G: UniformGenerator<Output = LinearCombination<Z>>,
> {
    circuit: &'a CircuitBuilder<Z>,
    logic_gate: LogicGate<'a, Z>,
    cache: Vec<G::Output>,
    have_bits: u32,
}

#[rustfmt::skip]
impl<
    'a,
    Z: IntegerRing,
    G: UniformGenerator<Output = LinearCombination<Z>>
> BinaryUniformDistribution<'a, Z, G> {
    fn useful_bits() -> u32 {
        if Z::MODULUS.count_ones() == 1 {
            Z::BITS
        } else {
            Z::BITS - 1
        }
    }
}

#[rustfmt::skip]
impl<
    'a,
    Z: IntegerRing,
    G: UniformGenerator<Output = LinearCombination<Z>>
> Distribution<'a, Z, G> for BinaryUniformDistribution<'a, Z, G> {
    type Output = LinearCombination<Z>;

    fn new(circuit: &'a CircuitBuilder<Z>) -> Self {
        Self {
            circuit,
            logic_gate: LogicGate::new(circuit),
            cache: vec![LinearCombination::<Z>::default(); Z::BITS as usize],
            have_bits: 0,
        }
    }

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        if self.have_bits == 0 {
            let scope = self.circuit.scope("BinaryUniformDistribution::sample");
            let generated = generator.generate();
            let mut p = Z::UNITY;
            let mut composed = LinearCombination::<Z>::default();
            for i in 0..Z::BITS {
                let digit = scope.auxiliary();
                self.cache[i as usize] = digit.into();
                composed += digit * Constant::from(p);
                p = p.double();
            }
            scope.constrain(composed, generated);
            let m1_gadget = (-Z::UNITY).gadget();
            self.logic_gate.check_less_or_equal(&self.cache, &m1_gadget);
            self.have_bits = Self::useful_bits();
        }
        let result = self.cache[(Self::useful_bits() - self.have_bits) as usize].clone();
        self.have_bits -= 1;
        result
    }
}
