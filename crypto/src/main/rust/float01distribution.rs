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

use crate::distribution::{Distribution, UniformGenerator};
use crate::float::{Float, FloatOn};
use crate::integer::Integer;
use crate::uniformintdistribution::UniformIntDistribution;
use core::marker::PhantomData;

// In range [0, 1)

pub struct Float01Distribution<
    F: Float,
    G: UniformGenerator<Output: Integer<CastUnsigned: FloatOn<F>>>,
> {
    uid: UniformIntDistribution<G>,
    phantom: PhantomData<F>,
}

impl<F: Float, G: UniformGenerator<Output: Integer<CastUnsigned: FloatOn<F>>>> Default
    for Float01Distribution<F, G>
{
    fn default() -> Self {
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

impl<F: Float, G: UniformGenerator<Output: Integer<CastUnsigned: FloatOn<F>>>> Distribution<G>
    for Float01Distribution<F, G>
{
    type Output = F;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        let one = <G::Output as Integer>::CastUnsigned::ONE;
        let s = F::ONE / (one << F::MANTISSA_DIGITS).float_on();
        let m = self.uid.sample(generator).float_on();
        s * m
    }

    fn reset(&mut self) {
        self.uid.reset()
    }
}
