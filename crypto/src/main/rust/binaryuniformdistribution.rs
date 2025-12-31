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

use crate::algebra::IntegerRing;
use crate::distribution::{Distribution, UniformGenerator};
use crate::integer::Integer;

pub struct BinaryUniformDistribution<G: UniformGenerator<Output: IntegerRing>> {
    cache: <G::Output as IntegerRing>::Int,
    have_bits: u32,
}

impl<G: UniformGenerator<Output: IntegerRing>> BinaryUniformDistribution<G> {
    pub const fn new() -> Self {
        Self {
            cache: <G::Output as IntegerRing>::Int::ZERO,
            have_bits: 0,
        }
    }

    fn useful_bits() -> u32 {
        if G::Output::MODULUS.count_ones() == 1 {
            G::Output::BITS
        } else {
            G::Output::BITS - 1
        }
    }
}

impl<G: UniformGenerator<Output: IntegerRing>> Default for BinaryUniformDistribution<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: UniformGenerator<Output: IntegerRing>> Distribution<G> for BinaryUniformDistribution<G> {
    type Output = G::Output;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        if self.have_bits == 0 {
            self.cache = generator.generate().canonical();
            self.have_bits = Self::useful_bits();
        }
        let result = self.cache & <G::Output as IntegerRing>::Int::LIMB_ONE;
        self.cache >>= <G::Output as IntegerRing>::Int::LIMB_ONE;
        self.have_bits -= 1;
        G::Output::with_limb(result)
    }

    fn reset(&mut self) {
        self.have_bits = 0
    }
}
