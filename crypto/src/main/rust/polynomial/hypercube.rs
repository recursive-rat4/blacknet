/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use crate::algebra::Semiring;
use crate::polynomial::MultivariatePolynomial;
use alloc::vec::Vec;
use core::iter::Map;
use core::marker::PhantomData;
use core::ops::Range;

/// An n-dimensional unit hypercube with a vertex at the origin in the coordinate system.
pub struct Hypercube<R: Semiring> {
    dimension: usize,
    vertices: usize,
    phantom: PhantomData<R>,
}

impl<R: Semiring> Hypercube<R> {
    /// Construct a new hypercube.
    pub const fn new(dimension: usize) -> Self {
        Self {
            dimension,
            vertices: 1 << dimension,
            phantom: PhantomData,
        }
    }

    /// Iterate indices of vertices.
    pub const fn iter_index(&self) -> Range<usize> {
        0..self.vertices
    }

    /// Iterate vertices.
    pub fn iter_vertex<Vertex: From<Vec<R>>>(
        &self,
    ) -> Map<Range<usize>, impl FnMut(usize) -> Vertex> {
        (0..self.vertices).map(move |index| {
            let mut coordinates = Vec::<R>::with_capacity(self.dimension);
            let mut s = self.vertices;
            for _ in 0..self.dimension {
                s >>= 1;
                if index & s == s {
                    coordinates.push(R::ONE)
                } else {
                    coordinates.push(R::ZERO)
                }
            }
            Vertex::from(coordinates)
        })
    }

    /// Iterate indices of vertices as order 2 tensor.
    pub fn iter_order2(
        &self,
        rows: usize,
        columns: usize,
    ) -> Map<Range<usize>, impl FnMut(usize) -> (usize, usize)> {
        debug_assert!(rows * columns == self.vertices);
        (0..self.vertices).map(move |index| (index / columns, index % columns))
    }

    /// Sum a polynomial over a unit hypercube.
    pub fn sum<P: MultivariatePolynomial<Coefficient = R, Point: From<Vec<R>>>>(
        polynomial: &P,
    ) -> R {
        Hypercube::new(polynomial.variables())
            .iter_vertex::<P::Point>()
            .map(|vertex| polynomial.point(&vertex))
            .sum()
    }
}
