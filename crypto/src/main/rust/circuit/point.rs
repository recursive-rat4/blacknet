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

use crate::circuit::circuitbuilder::{CircuitBuilder, LinearCombination, VariableKind};
use crate::ring::UnitalRing;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

pub struct Point<S: UnitalRing> {
    coordinates: Vec<LinearCombination<S>>,
}

impl<S: UnitalRing> Point<S> {
    pub fn allocate(circuit: &CircuitBuilder<S>, kind: VariableKind, dimension: usize) -> Self {
        let scope = circuit.scope("Point::allocate");
        Self {
            coordinates: (0..dimension)
                .map(|_| scope.variable(kind).into())
                .collect(),
        }
    }

    pub const fn dimension(&self) -> usize {
        self.coordinates.len()
    }

    pub const fn coordinates(&self) -> &Vec<LinearCombination<S>> {
        &self.coordinates
    }
}

impl<S: UnitalRing, const N: usize> From<[LinearCombination<S>; N]> for Point<S> {
    fn from(coordinates: [LinearCombination<S>; N]) -> Self {
        Self {
            coordinates: coordinates.into(),
        }
    }
}

impl<S: UnitalRing> From<Vec<LinearCombination<S>>> for Point<S> {
    #[inline]
    fn from(coordinates: Vec<LinearCombination<S>>) -> Self {
        Self { coordinates }
    }
}

impl<S: UnitalRing> From<Point<S>> for Vec<LinearCombination<S>> {
    #[inline]
    fn from(point: Point<S>) -> Self {
        point.coordinates
    }
}

impl<S: UnitalRing> Index<usize> for Point<S> {
    type Output = LinearCombination<S>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl<S: UnitalRing> IndexMut<usize> for Point<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coordinates[index]
    }
}
