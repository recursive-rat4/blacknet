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

use core::fmt::{Debug, Formatter, Result};
use core::ops::{Index, IndexMut};

#[derive(Eq, PartialEq)]
pub struct Point<S> {
    coordinates: Vec<S>,
}

impl<S> Point<S> {
    pub const fn dimension(&self) -> usize {
        self.coordinates.len()
    }

    pub const fn coordinates(&self) -> &Vec<S> {
        &self.coordinates
    }

    pub fn steal(self) -> Vec<S> {
        self.coordinates
    }
}

impl<S, const N: usize> From<[S; N]> for Point<S> {
    fn from(coordinates: [S; N]) -> Self {
        Self {
            coordinates: coordinates.into(),
        }
    }
}

impl<S> From<Vec<S>> for Point<S> {
    fn from(coordinates: Vec<S>) -> Self {
        Self { coordinates }
    }
}

impl<S: Debug> Debug for Point<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coordinates)
    }
}

impl<S> Index<usize> for Point<S> {
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl<S> IndexMut<usize> for Point<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coordinates[index]
    }
}
