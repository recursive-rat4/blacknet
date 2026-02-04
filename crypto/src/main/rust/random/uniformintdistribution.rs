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

use crate::integer::Integer;
use crate::random::{Distribution, UniformGenerator};
use core::ops::{Bound, RangeBounds};

pub struct UniformIntDistribution<G: UniformGenerator<Output: Integer>> {
    cache: <G::Output as Integer>::CastUnsigned,
    have_bits: u32,
    min: <G::Output as Integer>::CastUnsigned,
    length: <G::Output as Integer>::CastUnsigned,
    length_bits: u32,
    mask: <G::Output as Integer>::CastUnsigned,
}

impl<G: UniformGenerator<Output: Integer>> UniformIntDistribution<G> {
    pub fn new(range: impl RangeBounds<<G::Output as Integer>::CastUnsigned>) -> Self {
        let (min, length) = Self::parse(range);
        let length_bits = Self::length_bits(length);
        Self {
            cache: Default::default(),
            have_bits: 0,
            min,
            length,
            length_bits,
            mask: Self::mask(length_bits),
        }
    }

    pub fn set_range(&mut self, range: impl RangeBounds<<G::Output as Integer>::CastUnsigned>) {
        (self.min, self.length) = Self::parse(range);
        self.length_bits = Self::length_bits(self.length);
        self.mask = Self::mask(self.length_bits);
    }

    fn parse(
        range: impl RangeBounds<<G::Output as Integer>::CastUnsigned>,
    ) -> (
        <G::Output as Integer>::CastUnsigned,
        <G::Output as Integer>::CastUnsigned,
    ) {
        let min = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + <G::Output as Integer>::CastUnsigned::ONE,
            Bound::Unbounded => <G::Output as Integer>::CastUnsigned::MIN,
        };
        let max = match range.end_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n - <G::Output as Integer>::CastUnsigned::ONE,
            Bound::Unbounded => <G::Output as Integer>::CastUnsigned::MAX,
        };
        debug_assert!(max >= min);
        (min, max.wrapping_sub(min))
    }

    const fn useful_bits() -> u32 {
        G::Output::BITS
    }

    #[inline]
    fn length_bits(length: <G::Output as Integer>::CastUnsigned) -> u32 {
        G::Output::BITS - length.leading_zeros()
    }

    #[inline]
    fn mask(bits: u32) -> <G::Output as Integer>::CastUnsigned {
        if bits < <G::Output as Integer>::CastUnsigned::BITS {
            let one = <G::Output as Integer>::CastUnsigned::ONE;
            (one << bits) - one
        } else {
            <G::Output as Integer>::CastUnsigned::MAX
        }
    }

    fn next(&mut self, generator: &mut G) -> <G::Output as Integer>::CastUnsigned {
        if self.have_bits == 0 {
            self.cache = generator.generate().cast_unsigned();
            self.have_bits = Self::useful_bits();
        }

        if self.have_bits == self.length_bits {
            self.have_bits = 0;
            self.cache
        } else if self.have_bits > self.length_bits {
            let result = self.cache & self.mask;
            self.cache >>= self.length_bits;
            self.have_bits -= self.length_bits;
            result
        } else {
            let need_bits = self.length_bits - self.have_bits;
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
            if result <= self.length {
                return self.min.wrapping_add(result);
            }
        }
    }

    fn reset(&mut self) {
        self.have_bits = 0
    }
}
