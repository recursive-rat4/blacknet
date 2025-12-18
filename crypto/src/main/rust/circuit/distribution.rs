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

use crate::circuit::builder::CircuitBuilder;
use crate::distribution::{UniformDistribution, UniformGenerator};
use crate::semiring::Semiring;

pub trait Distribution<'a, 'b, R: Semiring, G: UniformGenerator> {
    type Output;

    fn new(circuit: &'a CircuitBuilder<'b, R>) -> Self;

    fn sample(&mut self, generator: &mut G) -> Self::Output;

    fn reset(&mut self);
}

impl<'a, 'b, R: Semiring, G: UniformGenerator> Distribution<'a, 'b, R, G>
    for UniformDistribution<G>
{
    type Output = G::Output;

    fn new(_: &'a CircuitBuilder<'b, R>) -> Self {
        Self::default()
    }

    #[inline]
    fn sample(&mut self, generator: &mut G) -> Self::Output {
        generator.generate()
    }

    #[inline]
    fn reset(&mut self) {}
}
