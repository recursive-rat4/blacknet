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
use crate::assigner::random::BinaryUniformDistribution;
use crate::random::{Distribution, UniformGenerator};

pub struct QuartaryUniformDistribution<'a, Z: IntegerModRing> {
    bud: BinaryUniformDistribution<'a, Z>,
}

impl<'a, Z: IntegerModRing> QuartaryUniformDistribution<'a, Z> {
    pub const fn new(assigment: &'a Assigment<Z>) -> Self {
        Self {
            bud: BinaryUniformDistribution::new(assigment),
        }
    }

    pub const fn reset(&mut self) {
        self.bud.reset()
    }
}

impl<'a, Z: IntegerModRing + Clone + Eq, G: UniformGenerator<Output = Z>> Distribution<Z, G>
    for QuartaryUniformDistribution<'a, Z>
where
    for<'b> &'b Z: RingOps<Z>,
{
    fn sample(&mut self, generator: &mut G) -> Z {
        self.bud.sample(generator).double() - self.bud.sample(generator)
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
