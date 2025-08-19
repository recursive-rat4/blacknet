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

pub struct TernaryUniformDistribution<G: UniformGenerator<Output: Integer>> {
    cache: G::Output,
    have_bits: u32,
}

impl<G: UniformGenerator<Output: Integer>> TernaryUniformDistribution<G> {
    const fn useful_bits() -> u32 {
        G::Output::BITS
    }
}

impl<G: UniformGenerator<Output: Integer>> Default for TernaryUniformDistribution<G> {
    fn default() -> Self {
        Self {
            cache: Default::default(),
            have_bits: 0,
        }
    }
}

impl<G: UniformGenerator<Output: Integer>> Distribution<G> for TernaryUniformDistribution<G> {
    type Output = G::Output;

    fn sample(&mut self, generator: &mut G) -> Self::Output {
        loop {
            if self.have_bits == 0 {
                self.cache = generator.generate();
                self.have_bits = Self::useful_bits();
            } else {
                debug_assert!(self.have_bits >= 2);
            }
            let result = self.cache & G::Output::LIMB_THREE;
            self.cache >>= G::Output::LIMB_TWO;
            self.have_bits -= 2;
            if result != G::Output::LIMB_THREE {
                return (result - G::Output::LIMB_ONE).into();
            }
        }
    }
}
