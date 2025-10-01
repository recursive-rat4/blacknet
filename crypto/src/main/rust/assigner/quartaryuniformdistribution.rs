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
use crate::assigner::binaryuniformdistribution::BinaryUniformDistribution;
use crate::assigner::distribution::Distribution;
use crate::distribution::UniformGenerator;
use crate::magma::AdditiveMagma;
use crate::ring::IntegerRing;

pub struct QuartaryUniformDistribution<'a, G: UniformGenerator<Output: IntegerRing>> {
    bud: BinaryUniformDistribution<'a, G>,
}

impl<'a, G: UniformGenerator<Output: IntegerRing>> Distribution<'a, G::Output, G>
    for QuartaryUniformDistribution<'a, G>
{
    type Output = G::Output;

    fn new(assigment: &'a Assigment<G::Output>) -> Self {
        Self {
            bud: BinaryUniformDistribution::new(assigment),
        }
    }

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        self.bud.sample(generator).double() - self.bud.sample(generator)
    }

    fn reset(&mut self) {
        self.bud.reset()
    }
}
