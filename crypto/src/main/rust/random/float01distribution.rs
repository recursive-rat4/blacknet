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
use crate::random::{Distribution, UniformGenerator, UniformIntDistribution};

/// Distribution of floats in range `[0, 1)`.
pub struct Float01Distribution<F: Float<Bits: UnsignedInteger>, G: UniformGenerator<Output = u8>> {
    uid: UniformIntDistribution<F::Bits, G>,
}

impl<F: Float<Bits: UnsignedInteger>, G: UniformGenerator<Output = u8>> Float01Distribution<F, G> {
    /// Construct the new distribution.
    pub fn new() -> Self {
        let bound = F::Bits::ONE << F::MANTISSA_DIGITS;
        Self {
            uid: UniformIntDistribution::new(..bound),
        }
    }
}

impl<F: Float<Bits: UnsignedInteger>, G: UniformGenerator<Output = u8>> Default
    for Float01Distribution<F, G>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Float<Bits: UnsignedInteger + Cast<F>>, G: UniformGenerator<Output = u8>> Distribution<G>
    for Float01Distribution<F, G>
{
    type Output = F;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        let s: F = (F::Bits::ONE << F::MANTISSA_DIGITS).cast().recip();
        let m: F = self.uid.sample(generator).cast();
        s * m
    }

    fn reset(&mut self) {
        self.uid.reset()
    }
}
