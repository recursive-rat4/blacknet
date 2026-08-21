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

use crate::algebra::{One, Zero};
use crate::polynomial::MultivariatePolynomial;
use alloc::vec::Vec;
use core::iter::{Map, Sum};
use core::ops::Range;

/// An n-dimensional unit hypercube with a vertex at the origin in the coordinate system.
pub struct Hypercube {
    dimension: u32,
    vertices: u32,
}

impl Hypercube {
    /// Construct a new hypercube.
    pub const fn new(dimension: u32) -> Self {
        Self {
            dimension,
            vertices: 1 << dimension,
        }
    }

    /// Iterate indices of vertices.
    pub const fn iter_index(&self) -> Range<u32> {
        0..self.vertices
    }

    /// Iterate vertices.
    pub fn iter_vertex<S: One + Zero, Vertex: From<Vec<S>>>(
        &self,
    ) -> Map<Range<u32>, impl FnMut(u32) -> Vertex> {
        (0..self.vertices).map(move |index| {
            let mut coordinates = Vec::<S>::with_capacity(self.dimension as usize);
            let mut s = self.vertices;
            for _ in 0..self.dimension {
                s >>= 1;
                if index & s == s {
                    coordinates.push(S::ONE)
                } else {
                    coordinates.push(S::ZERO)
                }
            }
            Vertex::from(coordinates)
        })
    }

    /// Iterate indices of vertices as order 2 tensor.
    pub fn iter_order2(
        &self,
        rows: u32,
        columns: u32,
    ) -> Map<Range<u32>, impl FnMut(u32) -> (u32, u32)> {
        debug_assert!(rows * columns == self.vertices);
        (0..self.vertices).map(move |index| (index / columns, index % columns))
    }

    /// Sum a polynomial over a unit hypercube.
    pub fn sum<
        R: One + Zero + Sum,
        P: MultivariatePolynomial<Coefficient = R, Point: From<Vec<R>>>,
    >(
        polynomial: &P,
    ) -> R {
        Hypercube::new(polynomial.variables())
            .iter_vertex::<R, P::Point>()
            .map(|vertex| polynomial.point(&vertex))
            .sum()
    }
}
