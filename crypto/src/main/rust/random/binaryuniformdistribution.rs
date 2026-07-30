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

use crate::algebra::IntegerModRing;
use crate::integer::Integer;
use crate::random::{Distribution, UniformGenerator};

/// Uniform distribution over subset `{0, 1}`.
pub struct BinaryUniformDistribution<Z: IntegerModRing> {
    cache: Z::Int,
    have_bits: u32,
}

impl<Z: IntegerModRing> BinaryUniformDistribution<Z> {
    /// Construct a new distribution.
    pub const fn new() -> Self {
        Self {
            cache: Z::Int::ZERO,
            have_bits: 0,
        }
    }

    /// Reset internal state.
    pub const fn reset(&mut self) {
        self.have_bits = 0
    }

    fn useful_bits() -> u32 {
        if Z::MODULUS.count_ones() == 1 {
            Z::BITS
        } else {
            Z::BITS - 1
        }
    }
}

impl<Z: IntegerModRing> Default for BinaryUniformDistribution<Z> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Z: IntegerModRing, G: UniformGenerator<Output = Z>> Distribution<Z, G>
    for BinaryUniformDistribution<Z>
{
    fn sample(&mut self, generator: &mut G) -> Z {
        if self.have_bits == 0 {
            self.cache = generator.generate().canonical();
            self.have_bits = Self::useful_bits();
        }
        let result = self.cache & Z::Int::LIMB_ONE;
        self.cache >>= Z::Int::LIMB_ONE;
        self.have_bits -= 1;
        Z::with_limb(result)
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
