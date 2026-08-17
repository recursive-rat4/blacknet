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

use crate::float::{Cast, Float};
use crate::integer::{Integer, UnsignedInteger};
use crate::random::{Distribution, UniformBitGenerator, UniformIntDistribution};

/// Distribution of floats in range `[0, 1)`.
pub struct Float01Distribution<F: Float<Bits: UnsignedInteger>> {
    uid: UniformIntDistribution<F::Bits>,
}

impl<F: Float<Bits: UnsignedInteger>> Float01Distribution<F> {
    /// Construct the new distribution.
    pub fn new() -> Self {
        let bound = F::Bits::ONE << F::MANTISSA_DIGITS;
        Self {
            uid: UniformIntDistribution::new(..bound),
        }
    }

    /// Reset internal state.
    pub const fn reset(&mut self) {
        self.uid.reset()
    }
}

impl<F: Float<Bits: UnsignedInteger>> Default for Float01Distribution<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Float<Bits: UnsignedInteger + Cast<F>>, G: UniformBitGenerator> Distribution<F, G>
    for Float01Distribution<F>
{
    fn sample(&mut self, generator: &mut G) -> F {
        let s: F = (F::Bits::ONE << F::MANTISSA_DIGITS).cast().recip();
        let m: F = self.uid.sample(generator).cast();
        s * m
    }

    #[inline]
    fn reset(&mut self) {
        self.reset()
    }
}
