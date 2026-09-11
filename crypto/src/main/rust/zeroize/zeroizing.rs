/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::zeroize::zeroize;
use bytemuck::Zeroable;
use core::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};

#[repr(transparent)]
pub struct Zeroizing<T: Zeroable>(T);

impl<T: Zeroable> Zeroizing<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn into_inner(self) -> T {
        let value = unsafe { ptr::read(&self.0) };
        mem::forget(self);
        value
    }
}

impl<T: Zeroable> From<T> for Zeroizing<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Zeroable> Drop for Zeroizing<T> {
    fn drop(&mut self) {
        zeroize(&mut self.0)
    }
}

impl<T: Zeroable> AsRef<T> for Zeroizing<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Zeroable> AsMut<T> for Zeroizing<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Zeroable> Deref for Zeroizing<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Zeroable> DerefMut for Zeroizing<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
