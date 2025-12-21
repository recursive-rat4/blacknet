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
use core::ops::{Bound, RangeBounds};

// In range [0, max]

pub struct UniformIntDistribution<G: UniformGenerator<Output: Integer>> {
    cache: <G::Output as Integer>::CastUnsigned,
    have_bits: u32,
    max: <G::Output as Integer>::CastUnsigned,
    output_bits: u32,
}

impl<G: UniformGenerator<Output: Integer>> UniformIntDistribution<G> {
    pub fn new(range: impl RangeBounds<<G::Output as Integer>::CastUnsigned>) -> Self {
        let max = Self::max(range);
        Self {
            cache: Default::default(),
            have_bits: 0,
            max,
            output_bits: Self::output_bits(max),
        }
    }

    pub fn set_range(&mut self, range: impl RangeBounds<<G::Output as Integer>::CastUnsigned>) {
        let max = Self::max(range);
        self.max = max;
        self.output_bits = Self::output_bits(max);
    }

    fn max(
        range: impl RangeBounds<<G::Output as Integer>::CastUnsigned>,
    ) -> <G::Output as Integer>::CastUnsigned {
        let zero = <G::Output as Integer>::CastUnsigned::ZERO;
        let one = <G::Output as Integer>::CastUnsigned::ONE;
        assert!(
            range.start_bound() == Bound::Included(&zero)
                || range.start_bound() == Bound::Unbounded,
            "Not implemented"
        );
        match range.end_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n - one,
            Bound::Unbounded => <G::Output as Integer>::CastUnsigned::MAX,
        }
    }

    const fn useful_bits() -> u32 {
        G::Output::BITS
    }

    #[inline]
    fn output_bits(max: <G::Output as Integer>::CastUnsigned) -> u32 {
        G::Output::BITS - max.leading_zeros()
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

impl<G: UniformGenerator<Output: Integer>> Default for UniformIntDistribution<G> {
    fn default() -> Self {
        Self::new(..)
    }
}

impl<G: UniformGenerator<Output: Integer>> Distribution<G> for UniformIntDistribution<G> {
    type Output = <G::Output as Integer>::CastUnsigned;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        loop {
            let result = self.next(generator);
            if result <= self.max {
                return result;
            }
        }
    }

    fn reset(&mut self) {
        self.have_bits = 0
    }
}
