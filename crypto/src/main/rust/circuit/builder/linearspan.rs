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

use crate::algebra::Semiring;
use crate::circuit::builder::LinearCombination;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::ops::Index;

/// A smallest linear subspace that contains an expression.
pub struct LinearSpan<R: Semiring> {
    vectors: Vec<LinearCombination<R>>,
}

impl<R: Semiring> LinearSpan<R> {
    pub const fn dimension(&self) -> usize {
        self.vectors.len()
    }
}

impl<R: Semiring> From<Vec<LinearCombination<R>>> for LinearSpan<R> {
    fn from(vectors: Vec<LinearCombination<R>>) -> Self {
        Self { vectors }
    }
}

impl<R: Semiring> From<VecDeque<LinearCombination<R>>> for LinearSpan<R> {
    fn from(vectors: VecDeque<LinearCombination<R>>) -> Self {
        Self {
            vectors: vectors.into(),
        }
    }
}

impl<R: Semiring> Index<usize> for LinearSpan<R> {
    type Output = LinearCombination<R>;

    fn index(&self, dimension: usize) -> &Self::Output {
        &self.vectors[dimension]
    }
}
