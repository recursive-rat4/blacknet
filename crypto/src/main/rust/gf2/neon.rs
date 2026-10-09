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

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use core::arch::arm::*;

#[inline(always)]
pub fn clsqr8(a: u8) -> u16 {
    unsafe {
        let a = vmov_n_p8(a);
        let c = vmull_p8(a, a);
        vgetq_lane_p16(c, 0)
    }
}

#[inline(always)]
pub fn clmul8(a: u8, b: u8) -> u16 {
    unsafe {
        let a = vmov_n_p8(a);
        let b = vmov_n_p8(b);
        let c = vmull_p8(a, b);
        vgetq_lane_p16(c, 0)
    }
}
