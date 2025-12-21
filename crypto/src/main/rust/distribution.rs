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

use core::marker::PhantomData;

pub trait UniformGenerator {
    type Output;

    fn generate(&mut self) -> Self::Output;
}

pub trait Distribution<G: UniformGenerator> {
    type Output;

    fn sample(&mut self, generator: &mut G) -> Self::Output;

    fn reset(&mut self);
}

pub struct UniformDistribution<G: UniformGenerator> {
    phantom: PhantomData<G>,
}

impl<G: UniformGenerator> Default for UniformDistribution<G> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
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
