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
use crate::integer::Integer;

// In range [0, bound)

pub struct UniformIntDistribution<G: UniformGenerator<Output: Integer>> {
    cache: <G::Output as Integer>::CastUnsigned,
    have_bits: u32,
    bound: <G::Output as Integer>::CastUnsigned,
    output_bits: u32,
}

impl<G: UniformGenerator<Output: Integer>> UniformIntDistribution<G> {
    pub fn new(bound: <G::Output as Integer>::CastUnsigned) -> Self {
        Self {
            cache: Default::default(),
            have_bits: 0,
            bound,
            output_bits: Self::output_bits(bound),
        }
    }

    const fn useful_bits() -> u32 {
        G::Output::BITS
    }

    #[inline]
    fn output_bits(bound: <G::Output as Integer>::CastUnsigned) -> u32 {
        let one = <G::Output as Integer>::CastUnsigned::ONE;
        G::Output::BITS - (bound - one).leading_zeros()
    }

    #[inline]
    fn mask(bits: u32) -> <G::Output as Integer>::CastUnsigned {
        let one = <G::Output as Integer>::CastUnsigned::ONE;
        (one << bits) - one
    }

    fn next(&mut self, generator: &mut G) -> <G::Output as Integer>::CastUnsigned {
        if self.have_bits == 0 {
            self.cache = generator.generate().cast_unsigned();
            self.have_bits = Self::useful_bits();
        }

        if self.have_bits == self.output_bits {
            self.have_bits = 0;
            self.cache
        } else if self.have_bits > self.output_bits {
            let mask = Self::mask(self.output_bits);
            let result = self.cache & mask;
            self.cache >>= self.output_bits;
            self.have_bits -= self.output_bits;
            result
        } else {
            let need_bits = self.output_bits - self.have_bits;
            let mut result = self.cache << need_bits;
            self.cache = generator.generate().cast_unsigned();
            self.have_bits = Self::useful_bits();
            let mask = Self::mask(need_bits);
            result |= self.cache & mask;
            self.cache >>= need_bits;
            self.have_bits -= need_bits;
            result
        }
    }
}

impl<G: UniformGenerator<Output: Integer>> Distribution<G> for UniformIntDistribution<G> {
    type Output = <G::Output as Integer>::CastUnsigned;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        loop {
            let result = self.next(generator);
            if result < self.bound {
                return result;
            }
        }
    }

    fn reset(&mut self) {
        self.have_bits = 0
    }
}
