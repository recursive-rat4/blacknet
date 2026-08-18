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

use crate::algebra::{One, Zero};
use crate::branchless::BlSelect;
use crate::random::{Distribution, UniformBitGenerator};

/// Uniform distribution over subset `{0, 1}`.
pub struct BinaryUniformDistribution {
    cache: u8,
    have_bits: u32,
}

impl BinaryUniformDistribution {
    /// Construct a new distribution.
    pub const fn new() -> Self {
        Self {
            cache: 0,
            have_bits: 0,
        }
    }

    /// Reset internal state.
    pub const fn reset(&mut self) {
        self.have_bits = 0
    }
}

impl Default for BinaryUniformDistribution {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: One + Zero + BlSelect<Output = S>, G: UniformBitGenerator> Distribution<S, G>
    for BinaryUniformDistribution
{
    fn sample(&mut self, generator: &mut G) -> S {
        if self.have_bits == 0 {
            self.cache = generator.generate();
            self.have_bits = u8::BITS;
        }
        let bit = self.cache & 1 != 0;
        self.cache >>= 1;
        self.have_bits -= 1;
        S::ZERO.bl_select(S::ONE, bit)
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
