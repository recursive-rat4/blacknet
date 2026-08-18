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

use crate::algebra::UnitalRing;
use crate::branchless::BlSelect;
use crate::random::{BinaryUniformDistribution, Distribution, UniformBitGenerator};

/// Uniform distribution over subset `{-1, 0, 1, 2}`.
pub struct QuartaryUniformDistribution {
    bud: BinaryUniformDistribution,
}

impl QuartaryUniformDistribution {
    /// Construct a new distribution.
    pub const fn new() -> Self {
        Self {
            bud: BinaryUniformDistribution::new(),
        }
    }

    /// Reset internal state.
    pub const fn reset(&mut self) {
        self.bud.reset()
    }
}

impl Default for QuartaryUniformDistribution {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: UnitalRing + BlSelect<Output = R>, G: UniformBitGenerator> Distribution<R, G>
    for QuartaryUniformDistribution
{
    fn sample(&mut self, generator: &mut G) -> R {
        let a = Distribution::<R, G>::sample(&mut self.bud, generator);
        let b = Distribution::<R, G>::sample(&mut self.bud, generator);
        a.double() - b
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
