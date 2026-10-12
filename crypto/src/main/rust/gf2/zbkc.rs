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

use core::arch::asm;

#[inline(always)]
pub fn clsqr8(a: u8) -> u16 {
    unsafe {
        let mut a = a as u16;
        asm!(
            "clmul {0}, {0}, {0}",
            inout(reg) a,
            options(nomem, nostack, preserves_flags, pure),
        );
        a
    }
}

#[inline(always)]
pub fn clsqr64(a: u64) -> [u64; 2] {
    unsafe {
        let mut l: u64;
        let mut h: u64;
        asm!(
            "clmul {1}, {0}, {0}",
            "clmulh {2}, {0}, {0}",
            in(reg) a,  out(reg) l, out(reg) h,
            options(nomem, nostack, preserves_flags, pure),
        );
        [l, h]
    }
}

#[inline(always)]
pub fn clsqr128(a: [u64; 2]) -> [u64; 4] {
    let [al, ah] = a;
    let [ll, lh] = clsqr64(al);
    let [hl, hh] = clsqr64(ah);
    [ll, lh, hl, hh]
}

#[inline(always)]
pub fn clsqr192(a: [u64; 3]) -> [u64; 6] {
    let [al, am, ah] = a;
    let [ll, lh] = clsqr64(al);
    let [ml, mh] = clsqr64(am);
    let [hl, hh] = clsqr64(ah);
    [ll, lh, ml, mh, hl, hh]
}

#[inline(always)]
pub fn clmul8(a: u8, b: u8) -> u16 {
    unsafe {
        let mut a = a as u16;
        asm!(
            "clmul {0}, {0}, {1}",
            inout(reg) a, in(reg) b,
            options(nomem, nostack, preserves_flags, pure),
        );
        a
    }
}

#[inline(always)]
pub fn clmul64(a: u64, b: u64) -> [u64; 2] {
    unsafe {
        let mut l: u64;
        let mut h: u64;
        asm!(
            "clmul {2}, {0}, {1}",
            "clmulh {3}, {0}, {1}",
            in(reg) a, in(reg) b, out(reg) l, out(reg) h,
            options(nomem, nostack, preserves_flags, pure),
        );
        [l, h]
    }
}

#[inline(always)]
pub fn clmul128(a: [u64; 2], b: [u64; 2]) -> [u64; 4] {
    // Long method
    let [lll, llh] = clmul64(a[0], b[0]);
    let [lhl, lhh] = clmul64(a[0], b[1]);
    let [hll, hlh] = clmul64(a[1], b[0]);
    let [hhl, hhh] = clmul64(a[1], b[1]);
    [lll, llh ^ lhl ^ hll, hhl ^ lhh ^ hlh, hhh]
}

#[inline(always)]
pub fn clmul192(a: [u64; 3], b: [u64; 3]) -> [u64; 6] {
    // Karatsuba method
    let p0 = clmul64(a[0], b[0]);
    let p1 = clmul64(a[1], b[1]);
    let p2 = clmul64(a[2], b[2]);
    let p01 = clmul64(a[0] ^ a[1], b[0] ^ b[1]);
    let p02 = clmul64(a[0] ^ a[2], b[0] ^ b[2]);
    let p12 = clmul64(a[1] ^ a[2], b[1] ^ b[2]);
    let t0 = p0;
    let t1 = [p0[0] ^ p01[0] ^ p1[0], p0[1] ^ p01[1] ^ p1[1]];
    let t2 = [
        p0[0] ^ p1[0] ^ p2[0] ^ p02[0],
        p0[1] ^ p1[1] ^ p2[1] ^ p02[1],
    ];
    let t3 = [p1[0] ^ p12[0] ^ p2[0], p1[1] ^ p12[1] ^ p2[1]];
    let t4 = p2;
    [
        t0[0],
        t0[1] ^ t1[0],
        t1[1] ^ t2[0],
        t2[1] ^ t3[0],
        t3[1] ^ t4[0],
        t4[1],
    ]
}
