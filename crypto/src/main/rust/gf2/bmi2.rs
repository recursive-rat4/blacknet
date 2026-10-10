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

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline(always)]
pub fn clsqr8(a: u8) -> u16 {
    unsafe {
        let a = a as u32;
        let c = _pdep_u32(a, 0x5555);
        c as u16
    }
}

#[inline(always)]
pub fn clsqr128(a: [u64; 2]) -> [u64; 4] {
    unsafe {
        let [al, ah] = a;
        let [ll, lh, hl, hh] = [al & 0xFFFFFFFF, al >> 32, ah & 0xFFFFFFFF, ah >> 32];
        let ll = _pdep_u64(ll, 0x5555555555555555);
        let lh = _pdep_u64(lh, 0x5555555555555555);
        let hl = _pdep_u64(hl, 0x5555555555555555);
        let hh = _pdep_u64(hh, 0x5555555555555555);
        [ll, lh, hl, hh]
    }
}

#[inline(always)]
pub fn clsqr192(a: [u64; 3]) -> [u64; 6] {
    unsafe {
        let [al, am, ah] = a;
        let [ll, lh, ml, mh, hl, hh] = [
            al & 0xFFFFFFFF,
            al >> 32,
            am & 0xFFFFFFFF,
            am >> 32,
            ah & 0xFFFFFFFF,
            ah >> 32,
        ];
        let ll = _pdep_u64(ll, 0x5555555555555555);
        let lh = _pdep_u64(lh, 0x5555555555555555);
        let ml = _pdep_u64(ml, 0x5555555555555555);
        let mh = _pdep_u64(mh, 0x5555555555555555);
        let hl = _pdep_u64(hl, 0x5555555555555555);
        let hh = _pdep_u64(hh, 0x5555555555555555);
        [ll, lh, ml, mh, hl, hh]
    }
}
