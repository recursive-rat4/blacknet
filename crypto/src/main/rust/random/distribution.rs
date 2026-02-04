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

use core::marker::PhantomData;

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

/// A probability distribution.
///
/// It takes a uniform generator as input and
/// possibly caches indeterminate values between samples.
pub trait Distribution<G: UniformGenerator> {
    /// Result type.
    type Output;

    /// Sample a random value.
    fn sample(&mut self, generator: &mut G) -> Self::Output;

    /// Reset internal caches to make the next samples independent of
    /// prior calls to generator.
    fn reset(&mut self);
}

/// Uniform distribution from uniform generator.
pub struct UniformDistribution<G: UniformGenerator> {
    phantom: PhantomData<G>,
}

impl<G: UniformGenerator> UniformDistribution<G> {
    /// Construct the new distribution.
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<G: UniformGenerator> Default for UniformDistribution<G> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<G: UniformGenerator> Distribution<G> for UniformDistribution<G> {
    type Output = G::Output;

    #[inline]
    fn sample(&mut self, generator: &mut G) -> Self::Output {
        generator.generate()
    }

    #[inline]
    fn reset(&mut self) {}
}
