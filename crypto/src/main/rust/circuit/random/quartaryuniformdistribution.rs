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

use crate::algebra::Double;
use crate::algebra::IntegerRing;
use crate::circuit::builder::{CircuitBuilder, LinearCombination};
use crate::circuit::random::{BinaryUniformDistribution, Distribution};
use crate::random::UniformGenerator;

pub struct QuartaryUniformDistribution<
    'a,
    'b,
    Z: IntegerRing,
    G: UniformGenerator<Output = LinearCombination<Z>>,
> {
    bud: BinaryUniformDistribution<'a, 'b, Z, G>,
}

#[rustfmt::skip]
impl<
    'a,
    'b,
    Z: IntegerRing + Eq,
    G: UniformGenerator<Output = LinearCombination<Z>>
> Distribution<'a, 'b, Z, G> for QuartaryUniformDistribution<'a, 'b, Z, G> {
    type Output = LinearCombination<Z>;

    fn new(circuit: &'a CircuitBuilder<'b, Z>) -> Self {
        Self {
            bud: BinaryUniformDistribution::new(circuit),
        }
    }

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        self.bud.sample(generator).double() - self.bud.sample(generator)
    }

    fn reset(&mut self) {
        self.bud.reset()
    }
}
