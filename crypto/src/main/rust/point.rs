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

use alloc::borrow::Borrow;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Deref, Index, IndexMut};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
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
}

impl<S, const N: usize> From<[S; N]> for Point<S> {
    fn from(coordinates: [S; N]) -> Self {
        Self {
            coordinates: coordinates.into(),
        }
    }
}

impl<S> From<Vec<S>> for Point<S> {
    #[inline]
    fn from(coordinates: Vec<S>) -> Self {
        Self { coordinates }
    }
}

impl<S> From<Point<S>> for Vec<S> {
    #[inline]
    fn from(point: Point<S>) -> Self {
        point.coordinates
    }
}

impl<S: Debug> Debug for Point<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coordinates)
    }
}

impl<S> AsRef<[S]> for Point<S> {
    #[inline]
    fn as_ref(&self) -> &[S] {
        &self.coordinates
    }
}

impl<S> Borrow<[S]> for Point<S> {
    #[inline]
    fn borrow(&self) -> &[S] {
        &self.coordinates
    }
}

impl<S> Deref for Point<S> {
    type Target = [S];

    #[inline]
    fn deref(&self) -> &[S] {
        &self.coordinates
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
