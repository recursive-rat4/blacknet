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

use crate::semiring::Presemiring;
use crate::vectordense::VectorDense;
use alloc::vec::Vec;
use core::cell::RefCell;

pub struct Assigment<R: Presemiring> {
    assigment: RefCell<Vec<R>>,
}

impl<R: Presemiring> Assigment<R> {
    pub fn new(variables: usize) -> Self {
        Self {
            assigment: RefCell::new(Vec::with_capacity(variables)),
        }
    }

    #[inline]
    pub fn extend<I: IntoIterator<Item = R>>(&self, iter: I) {
        self.assigment.borrow_mut().extend(iter)
    }

    #[inline]
    pub fn extend_from_slice(&self, slice: &[R]) {
        self.assigment.borrow_mut().extend_from_slice(slice)
    }

    #[inline]
    pub fn push(&self, value: R) {
        self.assigment.borrow_mut().push(value)
    }

    pub fn finish(self) -> VectorDense<R> {
        self.assigment.take().into()
    }
}
