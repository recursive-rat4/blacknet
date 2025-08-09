/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use crate::point::Point;
use crate::polynomial::Polynomial;
use crate::ring::Ring;
use core::marker::PhantomData;

pub struct Hypercube<R: Ring> {
    dimension: usize,
    vertices: usize,
    phantom: PhantomData<R>,
}

impl<R: Ring> Hypercube<R> {
    pub const fn new(dimension: usize) -> Self {
        Self {
            dimension,
            vertices: 1 << dimension,
            phantom: PhantomData,
        }
    }

    pub const fn iter_index(&self) -> IndexIterator {
        IndexIterator {
            index: 0,
            last: self.vertices,
        }
    }

    pub const fn iter_vertex(&self) -> VertexIterator<R> {
        VertexIterator {
            index: 0,
            last: self.vertices,
            dimension: self.dimension,
            phantom: PhantomData,
        }
    }

    pub const fn iter_rank2(&self, rows: usize, columns: usize) -> Rank2Iterator {
        Rank2Iterator {
            index: 0,
            last: self.vertices,
            _rows: rows,
            columns,
        }
    }

    pub fn sum<P: Polynomial<R>>(polynomial: &P) -> R {
        Hypercube::new(polynomial.variables())
            .iter_vertex()
            .map(|vertex| polynomial.point(&vertex))
            .sum()
    }
}

pub struct IndexIterator {
    index: usize,
    last: usize,
}

impl Iterator for IndexIterator {
    type Item = usize;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.last - self.index;
        (size, Some(size))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.last {
            let current = self.index;
            self.index += 1;
            Some(current)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for IndexIterator {
    fn len(&self) -> usize {
        self.last - self.index
    }
}

pub struct VertexIterator<R> {
    index: usize,
    last: usize,
    dimension: usize,
    phantom: PhantomData<R>,
}

impl<R: Ring> Iterator for VertexIterator<R> {
    type Item = Point<R>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.last - self.index;
        (size, Some(size))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.last {
            let mut coordinates = Vec::<R>::with_capacity(self.dimension);
            let mut s = self.last;
            for _ in 0..self.dimension {
                s >>= 1;
                if self.index & s == s {
                    coordinates.push(R::UNITY)
                } else {
                    coordinates.push(R::ZERO)
                }
            }
            self.index += 1;
            Some(coordinates.into())
        } else {
            None
        }
    }
}

impl<R: Ring> ExactSizeIterator for VertexIterator<R> {
    fn len(&self) -> usize {
        self.last - self.index
    }
}

pub struct Rank2Iterator {
    index: usize,
    last: usize,
    _rows: usize,
    columns: usize,
}

impl Iterator for Rank2Iterator {
    type Item = (usize, usize);

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.last - self.index;
        (size, Some(size))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.last {
            let tuple = (self.index / self.columns, self.index % self.columns);
            self.index += 1;
            Some(tuple)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Rank2Iterator {
    fn len(&self) -> usize {
        self.last - self.index
    }
}
