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
pub const fn clsqr8(a: u8) -> u16 {
    let mut c = a as u16;
    c = (c | c << 4) & 0x0F0F;
    c = (c | c << 2) & 0x3333;
    c = (c | c << 1) & 0x5555;
    c
}

#[inline(always)]
pub const fn clsqr32(a: u32) -> u64 {
    let mut c = a as u64;
    c = (c | c << 32) & 0x00000000FFFFFFFF;
    c = (c | c << 16) & 0x0000FFFF0000FFFF;
    c = (c | c << 8) & 0x00FF00FF00FF00FF;
    c = (c | c << 4) & 0x0F0F0F0F0F0F0F0F;
    c = (c | c << 2) & 0x3333333333333333;
    c = (c | c << 1) & 0x5555555555555555;
    c
}

#[inline(always)]
pub const fn clsqr128(a: [u64; 2]) -> [u64; 4] {
    let [al, ah] = a;
    let [ll, lh, hl, hh] = [al as u32, (al >> 32) as u32, ah as u32, (ah >> 32) as u32];
    let ll = clsqr32(ll);
    let lh = clsqr32(lh);
    let hl = clsqr32(hl);
    let hh = clsqr32(hh);
    [ll, lh, hl, hh]
}

#[inline(always)]
pub const fn clmul8(a: u8, b: u8) -> u16 {
    let mut a = a as u16;
    let mut b = b as u16;
    let mut c = 0;
    let mut i = 0;
    while i < u8::BITS {
        let mask = (a & 1).wrapping_neg();
        c ^= b & mask;
        a >>= 1;
        b <<= 1;
        i += 1;
    }
    c
}

#[inline(always)]
pub const fn clmul64(a: u64, b: u64) -> [u64; 2] {
    let [mut l, mut h] = [0, 0];
    let mask = (a & 1).wrapping_neg();
    l ^= mask & b;
    let mut i = 1;
    while i < 64 {
        let mask = (a >> i & 1).wrapping_neg();
        l ^= mask & b << i;
        h ^= mask & b >> (64 - i);
        i += 1;
    }
    [l, h]
}

#[inline(always)]
pub const fn clmul128(a: [u64; 2], b: [u64; 2]) -> [u64; 4] {
    // Karatsuba method
    let [al, ah] = a;
    let [bl, bh] = b;
    let [ta, tb] = [al ^ ah, bl ^ bh];
    let [ll, lh] = clmul64(al, bl);
    let [hl, hh] = clmul64(ah, bh);
    let [tl, th] = clmul64(ta, tb);
    [ll, lh ^ ll ^ hl ^ tl, hl ^ lh ^ hh ^ th, hh]
}
