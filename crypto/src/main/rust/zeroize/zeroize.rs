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

use alloc::{string::String, vec::Vec};
use bytemuck::Zeroable;

#[inline(always)]
fn black_box<T: ?Sized>(x: &T) {
    unsafe {
        core::arch::asm!(
            "# zeroize::black_box {}",
            in(reg) x as *const T as *const (),
            options(readonly, preserves_flags, nostack),
        );
    }
}

/// Zeroize a value.
pub fn zeroize<T: Copy + Zeroable>(x: &mut T) {
    unsafe { core::ptr::write_bytes(x, 0, 1) };
    black_box(x);
}

/// Zeroize a slice of values.
pub fn zeroize_slice<T: Copy + Zeroable>(x: &mut [T]) {
    unsafe { core::ptr::write_bytes(x.as_mut_ptr(), 0, x.len()) };
    black_box(x);
}

/// Zeroize a string.
#[inline]
pub fn zeroize_string(x: &mut String) {
    zeroize_vec(unsafe { x.as_mut_vec() });
}

/// Zeroize a vector of values.
#[inline]
pub fn zeroize_vec<T: Copy + Zeroable>(x: &mut Vec<T>) {
    x.clear();
    zeroize_slice(x.spare_capacity_mut());
}

/// Overwrite with default value.
///
/// For types that don't implement `Zeroable`.
pub fn zeroize_with_default<T: Copy + Default>(x: &mut T) {
    let y = T::default();
    unsafe { core::ptr::copy_nonoverlapping(&y, x, 1) };
    black_box(x);
}
