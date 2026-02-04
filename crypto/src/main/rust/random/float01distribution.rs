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
use crate::integer::Integer;
use crate::random::{Distribution, UniformGenerator, UniformIntDistribution};
use core::marker::PhantomData;

/// Distribution of floats in range `[0, 1)`.
pub struct Float01Distribution<
    F: Float,
    G: UniformGenerator<Output: Integer<CastUnsigned: Cast<F>>>,
> {
    uid: UniformIntDistribution<G>,
    phantom: PhantomData<F>,
}

impl<F: Float, G: UniformGenerator<Output: Integer<CastUnsigned: Cast<F>>>>
    Float01Distribution<F, G>
{
    /// Construct the new distribution.
    pub fn new() -> Self {
        const {
            assert!(size_of::<F>() <= size_of::<G::Output>());
        };
        let zero = <G::Output as Integer>::CastUnsigned::ZERO;
        let one = <G::Output as Integer>::CastUnsigned::ONE;
        let bound = one << F::MANTISSA_DIGITS;
        Self {
            uid: UniformIntDistribution::new(zero..bound),
            phantom: PhantomData,
        }
    }
}

impl<F: Float, G: UniformGenerator<Output: Integer<CastUnsigned: Cast<F>>>> Default
    for Float01Distribution<F, G>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F: Float, G: UniformGenerator<Output: Integer<CastUnsigned: Cast<F>>>> Distribution<G>
    for Float01Distribution<F, G>
{
    type Output = F;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        let one = <G::Output as Integer>::CastUnsigned::ONE;
        let s: F = (one << F::MANTISSA_DIGITS).cast().recip();
        let m: F = self.uid.sample(generator).cast();
        s * m
    }

    fn reset(&mut self) {
        self.uid.reset()
    }
}
