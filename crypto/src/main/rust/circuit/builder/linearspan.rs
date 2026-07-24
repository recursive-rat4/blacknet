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

use crate::algebra::UnitalSemiring;
use crate::circuit::builder::LinearCombination;
use alloc::borrow::Borrow;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::ops::{Deref, Index};

/// A smallest linear subspace that contains an expression.
pub struct LinearSpan<R: UnitalSemiring> {
    vectors: Vec<LinearCombination<R>>,
}

impl<R: UnitalSemiring> LinearSpan<R> {
    pub const fn dimension(&self) -> usize {
        self.vectors.len()
    }
}

impl<R: UnitalSemiring> From<Vec<LinearCombination<R>>> for LinearSpan<R> {
    fn from(vectors: Vec<LinearCombination<R>>) -> Self {
        Self { vectors }
    }
}

impl<R: UnitalSemiring> From<VecDeque<LinearCombination<R>>> for LinearSpan<R> {
    fn from(vectors: VecDeque<LinearCombination<R>>) -> Self {
        Self {
            vectors: vectors.into(),
        }
    }
}

impl<R: UnitalSemiring> AsRef<[LinearCombination<R>]> for LinearSpan<R> {
    #[inline]
    fn as_ref(&self) -> &[LinearCombination<R>] {
        &self.vectors
    }
}

impl<R: UnitalSemiring> Borrow<[LinearCombination<R>]> for LinearSpan<R> {
    #[inline]
    fn borrow(&self) -> &[LinearCombination<R>] {
        &self.vectors
    }
}

impl<R: UnitalSemiring> Deref for LinearSpan<R> {
    type Target = [LinearCombination<R>];

    #[inline]
    fn deref(&self) -> &[LinearCombination<R>] {
        &self.vectors
    }
}

impl<R: UnitalSemiring> Index<usize> for LinearSpan<R> {
    type Output = LinearCombination<R>;

    fn index(&self, dimension: usize) -> &Self::Output {
        &self.vectors[dimension]
    }
}

impl<R: UnitalSemiring> IntoIterator for LinearSpan<R> {
    type Item = LinearCombination<R>;
    type IntoIter = alloc::vec::IntoIter<LinearCombination<R>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.vectors.into_iter()
    }
}

impl<'a, R: UnitalSemiring> IntoIterator for &'a LinearSpan<R> {
    type Item = &'a LinearCombination<R>;
    type IntoIter = core::slice::Iter<'a, LinearCombination<R>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.vectors.iter()
    }
}
