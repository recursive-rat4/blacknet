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
use core::ops::Deref;

#[repr(transparent)]
pub struct Zeroizing<T: Zeroable>(T);

impl<T: Zeroable> Drop for Zeroizing<T> {
    fn drop(&mut self) {
        zeroize(&mut self.0)
    }
}

impl<T: Zeroable> Deref for Zeroizing<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}
