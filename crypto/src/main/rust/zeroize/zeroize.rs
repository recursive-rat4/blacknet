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

#[inline(always)]
fn black_box<T: ?Sized>(x: &T) {
    unsafe {
        core::arch::asm!(
            "# black_box {}",
            in(reg) x as *const T as *const (),
            options(readonly, preserves_flags, nostack),
        );
    }
}

pub fn zeroize<T: bytemuck::Zeroable>(x: &mut T) {
    unsafe {
        core::ptr::drop_in_place(x);
        core::ptr::write_bytes(x, 0, 1);
    }
    black_box(x);
}

pub fn zeroize_slice<T: bytemuck::Zeroable>(x: &mut [T]) {
    unsafe {
        core::ptr::drop_in_place(x);
        core::ptr::write_bytes(x.as_mut_ptr(), 0, x.len());
    }
    black_box(x);
}

#[inline]
pub fn zeroize_string(x: &mut alloc::string::String) {
    zeroize_vec(unsafe { x.as_mut_vec() });
}

#[inline]
pub fn zeroize_vec<T: bytemuck::Zeroable>(x: &mut alloc::vec::Vec<T>) {
    x.clear();
    zeroize_slice(x.spare_capacity_mut());
}

pub fn zeroize_with_default<T: Default>(x: &mut T) {
    let y = T::default();
    unsafe {
        core::ptr::drop_in_place(x);
        core::ptr::copy_nonoverlapping(&y, x, 1);
        core::mem::forget(y);
    }
    black_box(x);
}
