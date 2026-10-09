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
pub fn clmul8(a: u8, b: u8) -> u16 {
    unsafe {
        let a = _mm_cvtsi32_si128(a as i32);
        let b = _mm_cvtsi32_si128(b as i32);
        let c = _mm_clmulepi64_si128(a, b, 0);
        let c: u128 = core::mem::transmute(c);
        c as u16
    }
}

#[inline(always)]
pub fn clmul128(a: [u64; 2], b: [u64; 2]) -> [u64; 4] {
    unsafe {
        // Long method
        let a = _mm_loadu_si128(a.as_ptr() as *const __m128i);
        let b = _mm_loadu_si128(b.as_ptr() as *const __m128i);
        let ll = _mm_clmulepi64_si128(a, b, 0);
        let lh = _mm_clmulepi64_si128(a, b, 1);
        let hl = _mm_clmulepi64_si128(a, b, 16);
        let hh = _mm_clmulepi64_si128(a, b, 17);
        let [lll, llh]: [u64; 2] = core::mem::transmute(ll);
        let [lhl, lhh]: [u64; 2] = core::mem::transmute(lh);
        let [hll, hlh]: [u64; 2] = core::mem::transmute(hl);
        let [hhl, hhh]: [u64; 2] = core::mem::transmute(hh);
        [lll, llh ^ lhl ^ hll, hhl ^ lhh ^ hlh, hhh]
    }
}
