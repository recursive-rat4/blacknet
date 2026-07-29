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

use crate::algebra::Set;
use crate::assigner::assigment::Assigment;
use crate::random::{UniformDistribution, UniformGenerator};

pub trait Distribution<'a, S: Set, Sample, Generator> {
    fn new(assigment: &'a Assigment<S>) -> Self;

    fn sample(&mut self, generator: &mut Generator) -> Sample;

    fn reset(&mut self);
}

impl<'a, S: Set, G: UniformGenerator> Distribution<'a, S, G::Output, G> for UniformDistribution {
    fn new(_: &'a Assigment<S>) -> Self {
        Self
    }

    #[inline]
    fn sample(&mut self, generator: &mut G) -> G::Output {
        generator.generate()
    }

    #[inline]
    fn reset(&mut self) {}
}
