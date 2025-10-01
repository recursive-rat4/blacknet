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

use crate::binaryuniformdistribution::BinaryUniformDistribution;
use crate::distribution::{Distribution, UniformGenerator};
use crate::magma::AdditiveMagma;
use crate::ring::IntegerRing;

pub struct QuartaryUniformDistribution<G: UniformGenerator<Output: IntegerRing>> {
    bud: BinaryUniformDistribution<G>,
}

impl<G: UniformGenerator<Output: IntegerRing>> Default for QuartaryUniformDistribution<G> {
    fn default() -> Self {
        Self {
            bud: Default::default(),
        }
    }
}

impl<G: UniformGenerator<Output: IntegerRing>> Distribution<G> for QuartaryUniformDistribution<G> {
    type Output = G::Output;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        self.bud.sample(generator).double() - self.bud.sample(generator)
    }

    fn reset(&mut self) {
        self.bud.reset()
    }
}
