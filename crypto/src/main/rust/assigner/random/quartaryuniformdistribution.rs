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

use crate::algebra::{Double, IntegerModRing, RingOps};
use crate::assigner::assigment::Assigment;
use crate::assigner::random::{BinaryUniformDistribution, Distribution};
use crate::random::UniformGenerator;

pub struct QuartaryUniformDistribution<'a, G: UniformGenerator<Output: IntegerModRing>> {
    bud: BinaryUniformDistribution<'a, G>,
}

impl<'a, G: UniformGenerator<Output: IntegerModRing + Clone + Eq>> Distribution<'a, G::Output, G>
    for QuartaryUniformDistribution<'a, G>
where
    for<'b> &'b G::Output: RingOps<G::Output>,
{
    type Output = G::Output;

    fn new(assigment: &'a Assigment<G::Output>) -> Self {
        Self {
            bud: BinaryUniformDistribution::<G>::new(assigment),
        }
    }

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        self.bud.sample(generator).double() - self.bud.sample(generator)
    }

    fn reset(&mut self) {
        self.bud.reset()
    }
}
