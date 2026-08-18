/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::IntegerModRing;
use crate::integer::Integer;
use crate::random::{Distribution, UniformBitGenerator, UniformIntDistribution};

/// Uniform distribution over ring `ℤ/q`.
pub struct UniformModDistribution<Z: IntegerModRing> {
    uid: UniformIntDistribution<Z::Int>,
}

impl<Z: IntegerModRing> UniformModDistribution<Z> {
    pub fn new() -> Self {
        let min = Z::Int::ZERO;
        let max = const { Z::MAX_CANONICAL.expect("finite ring") };
        Self {
            uid: UniformIntDistribution::new(min..=max),
        }
    }

    pub const fn reset(&mut self) {
        self.uid.reset()
    }
}

impl<Z: IntegerModRing> Default for UniformModDistribution<Z> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Z: IntegerModRing, G: UniformBitGenerator> Distribution<Z, G> for UniformModDistribution<Z> {
    fn sample(&mut self, generator: &mut G) -> Z {
        let int = self.uid.sample(generator);
        Z::with_int(int)
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
