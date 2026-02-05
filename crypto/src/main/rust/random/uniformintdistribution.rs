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
use core::borrow::BorrowMut;
use core::marker::PhantomData;
use core::ops::{Bound, RangeBounds};

pub struct UniformIntDistribution<I: Integer, G: UniformGenerator<Output = u8>> {
    min: I,
    length: I::CastUnsigned,
    length_bytes: usize,
    mask: I::CastUnsigned,
    phantom: PhantomData<G>,
}

impl<I: Integer, G: UniformGenerator<Output = u8>> UniformIntDistribution<I, G> {
    pub fn new(range: impl RangeBounds<I>) -> Self {
        let (min, length) = Self::parse(range);
        let length_bits = Self::length_bits(length);
        Self {
            min,
            length,
            length_bytes: Self::length_bytes(length_bits),
            mask: Self::mask(length_bits),
            phantom: PhantomData,
        }
    }

    pub fn set_range(&mut self, range: impl RangeBounds<I>) {
        (self.min, self.length) = Self::parse(range);
        let length_bits = Self::length_bits(self.length);
        self.length_bytes = Self::length_bytes(length_bits);
        self.mask = Self::mask(length_bits);
    }

    fn parse(range: impl RangeBounds<I>) -> (I, I::CastUnsigned) {
        let min = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + I::ONE,
            Bound::Unbounded => I::MIN,
        };
        let max = match range.end_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n - I::ONE,
            Bound::Unbounded => I::MAX,
        };
        debug_assert!(max >= min);
        (min, max.wrapping_sub(min).cast_unsigned())
    }

    #[inline]
    fn length_bits(length: I::CastUnsigned) -> u32 {
        I::CastUnsigned::BITS - length.leading_zeros()
    }

    const fn length_bytes(length_bits: u32) -> usize {
        (length_bits.next_multiple_of(8) >> 3) as usize
    }

    #[inline]
    fn mask(bits: u32) -> I::CastUnsigned {
        if bits < I::CastUnsigned::BITS {
            let one = I::CastUnsigned::ONE;
            (one << bits) - one
        } else {
            I::CastUnsigned::MAX
        }
    }

    fn next(&mut self, generator: &mut G) -> I::CastUnsigned {
        let mut bytes = <I::CastUnsigned as Integer>::Bytes::default();
        generator.fill(&mut bytes.borrow_mut()[..self.length_bytes]);
        let int = I::CastUnsigned::from_le_bytes(bytes);
        int & self.mask
    }
}

impl<I: Integer, G: UniformGenerator<Output = u8>> Default for UniformIntDistribution<I, G> {
    fn default() -> Self {
        Self::new(..)
    }
}

impl<I: Integer, G: UniformGenerator<Output = u8>> Distribution<G>
    for UniformIntDistribution<I, G>
{
    type Output = I;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        loop {
            let result = self.next(generator);
            if result <= self.length {
                let bytes = result.to_le_bytes();
                let result = I::from_le_bytes(bytes);
                return self.min.wrapping_add(result);
            }
        }
    }

    fn reset(&mut self) {}
}
