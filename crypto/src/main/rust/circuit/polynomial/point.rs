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

use crate::algebra::Semiring;
use crate::circuit::builder::{CircuitBuilder, LinearCombination, VariableKind};
use alloc::borrow::{Borrow, BorrowMut};
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut, Index, IndexMut};

pub struct Point<S: Semiring> {
    coordinates: Vec<LinearCombination<S>>,
}

impl<S: Semiring> Point<S> {
    pub const fn new(coordinates: Vec<LinearCombination<S>>) -> Self {
        Self { coordinates }
    }

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

impl<S: Semiring, const N: usize> From<[LinearCombination<S>; N]> for Point<S> {
    fn from(coordinates: [LinearCombination<S>; N]) -> Self {
        Self {
            coordinates: coordinates.into(),
        }
    }
}

impl<S: Semiring> From<Vec<LinearCombination<S>>> for Point<S> {
    #[inline]
    fn from(coordinates: Vec<LinearCombination<S>>) -> Self {
        Self { coordinates }
    }
}

impl<S: Semiring> From<Point<S>> for Vec<LinearCombination<S>> {
    #[inline]
    fn from(point: Point<S>) -> Self {
        point.coordinates
    }
}

impl<S: Semiring> AsRef<[LinearCombination<S>]> for Point<S> {
    #[inline]
    fn as_ref(&self) -> &[LinearCombination<S>] {
        &self.coordinates
    }
}

impl<S: Semiring> AsMut<[LinearCombination<S>]> for Point<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [LinearCombination<S>] {
        self
    }
}

impl<S: Semiring> Borrow<[LinearCombination<S>]> for Point<S> {
    #[inline]
    fn borrow(&self) -> &[LinearCombination<S>] {
        &self.coordinates
    }
}

impl<S: Semiring> BorrowMut<[LinearCombination<S>]> for Point<S> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [LinearCombination<S>] {
        &mut self.coordinates
    }
}

impl<S: Semiring> Deref for Point<S> {
    type Target = [LinearCombination<S>];

    #[inline]
    fn deref(&self) -> &[LinearCombination<S>] {
        &self.coordinates
    }
}

impl<S: Semiring> DerefMut for Point<S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coordinates
    }
}

impl<S: Semiring> Index<usize> for Point<S> {
    type Output = LinearCombination<S>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl<S: Semiring> IndexMut<usize> for Point<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coordinates[index]
    }
}

impl<S: Semiring> IntoIterator for Point<S> {
    type Item = LinearCombination<S>;
    type IntoIter = alloc::vec::IntoIter<LinearCombination<S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coordinates.into_iter()
    }
}

impl<'a, S: Semiring> IntoIterator for &'a Point<S> {
    type Item = &'a LinearCombination<S>;
    type IntoIter = core::slice::Iter<'a, LinearCombination<S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coordinates.iter()
    }
}
