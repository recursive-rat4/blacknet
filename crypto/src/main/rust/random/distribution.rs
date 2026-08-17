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

/// Generator of uniformly distributed values.
pub trait UniformGenerator {
    /// The type of generated values.
    type Output;

    /// Generate a single value.
    fn generate(&mut self) -> Self::Output;

    /// Generate a sequence of values.
    fn fill(&mut self, sequence: &mut [Self::Output]) {
        for i in sequence {
            *i = self.generate()
        }
    }
}

/// Generator of uniformly distributed bytes.
pub trait UniformBitGenerator: UniformGenerator<Output = u8> {}

impl<G: UniformGenerator<Output = u8>> UniformBitGenerator for G {}

/// A probability distribution.
///
/// It takes a generator as input and
/// possibly caches indeterminate values between samples.
pub trait Distribution<Sample, Generator> {
    /// Sample a random value.
    fn sample(&mut self, generator: &mut Generator) -> Sample;

    /// Reset internal caches to make the next samples independent of
    /// prior calls to generator.
    fn reset(&mut self);
}

/// Uniform distribution from uniform generator.
#[derive(Default)]
pub struct UniformDistribution;

impl UniformDistribution {
    /// Construct the new distribution.
    pub const fn new() -> Self {
        Self
    }

    /// Reset internal state.
    pub const fn reset(&mut self) {}
}

impl<G: UniformGenerator> Distribution<G::Output, G> for UniformDistribution {
    #[inline]
    fn sample(&mut self, generator: &mut G) -> G::Output {
        generator.generate()
    }

    #[inline]
    fn reset(&mut self) {}
}
