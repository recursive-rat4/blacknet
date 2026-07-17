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
use core::array;
use core::mem::transmute;

//RUST https://github.com/rust-lang/rust/issues/111147
#[inline(always)]
const fn mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[inline(always)]
fn rotate_right_32(x: __m256i) -> __m256i {
    const MASK: i32 = mm_shuffle(2, 3, 0, 1);
    unsafe { _mm256_shuffle_epi32(x, MASK) }
}

#[inline(always)]
fn rotate_right_24(x: __m256i) -> __m256i {
    #[rustfmt::skip]
    const MASK: __m256i = unsafe {
        transmute::<[u8; 32], __m256i>([
            3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10,
            3, 4, 5, 6, 7, 0, 1, 2, 11, 12, 13, 14, 15, 8, 9, 10,
        ])
    };
    unsafe { _mm256_shuffle_epi8(x, MASK) }
}

#[inline(always)]
fn rotate_right_16(x: __m256i) -> __m256i {
    #[rustfmt::skip]
    const MASK: __m256i = unsafe {
        transmute::<[u8; 32], __m256i>([
            2, 3, 4, 5, 6, 7, 0, 1, 10, 11, 12, 13, 14, 15, 8, 9,
            2, 3, 4, 5, 6, 7, 0, 1, 10, 11, 12, 13, 14, 15, 8, 9,
        ])
    };
    unsafe { _mm256_shuffle_epi8(x, MASK) }
}

#[inline(always)]
fn rotate_right_63(x: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_add_epi64(x, x), _mm256_srli_epi64(x, 63)) }
}

#[inline(always)]
fn diagonalize(a: __m256i, c: __m256i, d: __m256i) -> (__m256i, __m256i, __m256i) {
    unsafe {
        (
            _mm256_permute4x64_epi64(a, mm_shuffle(2, 1, 0, 3)),
            _mm256_permute4x64_epi64(c, mm_shuffle(0, 3, 2, 1)),
            _mm256_permute4x64_epi64(d, mm_shuffle(1, 0, 3, 2)),
        )
    }
}

#[inline(always)]
fn undiagonalize(a: __m256i, c: __m256i, d: __m256i) -> (__m256i, __m256i, __m256i) {
    unsafe {
        (
            _mm256_permute4x64_epi64(a, mm_shuffle(0, 3, 2, 1)),
            _mm256_permute4x64_epi64(c, mm_shuffle(2, 1, 0, 3)),
            _mm256_permute4x64_epi64(d, mm_shuffle(1, 0, 3, 2)),
        )
    }
}

#[inline(always)]
fn g1(
    a: __m256i,
    b: __m256i,
    c: __m256i,
    d: __m256i,
    x: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    unsafe {
        let a = _mm256_add_epi64(_mm256_add_epi64(a, b), x);
        let d = rotate_right_32(_mm256_xor_si256(d, a));
        let c = _mm256_add_epi64(c, d);
        let b = rotate_right_24(_mm256_xor_si256(b, c));
        (a, b, c, d)
    }
}

#[inline(always)]
fn g2(
    a: __m256i,
    b: __m256i,
    c: __m256i,
    d: __m256i,
    y: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    unsafe {
        let a = _mm256_add_epi64(_mm256_add_epi64(a, b), y);
        let d = rotate_right_16(_mm256_xor_si256(d, a));
        let c = _mm256_add_epi64(c, d);
        let b = rotate_right_63(_mm256_xor_si256(b, c));
        (a, b, c, d)
    }
}

#[inline(always)]
fn r_1_1(
    a: __m256i,
    b: __m256i,
    c: __m256i,
    d: __m256i,
    x: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    g1(a, b, c, d, x)
}

#[inline(always)]
fn r_1_2(
    a: __m256i,
    b: __m256i,
    c: __m256i,
    d: __m256i,
    y: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    g2(a, b, c, d, y)
}

#[inline(always)]
fn r_2_1(
    a: __m256i,
    b: __m256i,
    c: __m256i,
    d: __m256i,
    x: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let (a, c, d) = diagonalize(a, c, d);
    g1(a, b, c, d, x)
}

#[inline(always)]
fn r_2_2(
    a: __m256i,
    b: __m256i,
    c: __m256i,
    d: __m256i,
    y: __m256i,
) -> (__m256i, __m256i, __m256i, __m256i) {
    let (a, b, c, d) = g2(a, b, c, d, y);
    let (a, c, d) = undiagonalize(a, c, d);
    (a, b, c, d)
}

#[inline(always)]
fn broadcast_input(input: &[u64; 16]) -> [__m256i; 8] {
    let ptr = input.as_ptr() as *const __m128i;
    array::from_fn(|i| unsafe { _mm256_broadcastsi128_si256(_mm_loadu_si128(ptr.add(i))) })
}

#[inline(always)]
fn input_1_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[0], input[1]),
            _mm256_unpacklo_epi64(input[2], input[3]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_1_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[0], input[1]),
            _mm256_unpackhi_epi64(input[2], input[3]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_1_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[7], input[4]),
            _mm256_unpacklo_epi64(input[5], input[6]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_1_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[7], input[4]),
            _mm256_unpackhi_epi64(input[5], input[6]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_2_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[7], input[2]),
            _mm256_unpackhi_epi64(input[4], input[6]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_2_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[5], input[4]),
            _mm256_alignr_epi8(input[3], input[7], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_2_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[2], input[0]),
            _mm256_blend_epi32(input[5], input[0], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_2_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[6], input[1], 8),
            _mm256_blend_epi32(input[3], input[1], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_3_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[6], input[5], 8),
            _mm256_unpackhi_epi64(input[2], input[7]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_3_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[4], input[0]),
            _mm256_blend_epi32(input[6], input[1], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_3_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[5], input[4], 8),
            _mm256_unpackhi_epi64(input[1], input[3]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_3_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[2], input[7]),
            _mm256_blend_epi32(input[0], input[3], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_4_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[3], input[1]),
            _mm256_unpackhi_epi64(input[6], input[5]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_4_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[4], input[0]),
            _mm256_unpacklo_epi64(input[6], input[7]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_4_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[1], input[7], 8),
            _mm256_shuffle_epi32(input[2], mm_shuffle(1, 0, 3, 2)),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_4_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[4], input[3]),
            _mm256_unpacklo_epi64(input[5], input[0]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_5_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[4], input[2]),
            _mm256_unpacklo_epi64(input[1], input[5]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_5_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_blend_epi32(input[3], input[0], 0x33),
            _mm256_blend_epi32(input[7], input[2], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_5_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[7], input[1], 8),
            _mm256_alignr_epi8(input[3], input[5], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_5_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[6], input[0]),
            _mm256_unpacklo_epi64(input[6], input[4]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_6_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[1], input[3]),
            _mm256_unpacklo_epi64(input[0], input[4]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_6_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[6], input[5]),
            _mm256_unpackhi_epi64(input[5], input[1]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_6_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[2], input[0], 8),
            _mm256_unpackhi_epi64(input[3], input[7]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_6_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[4], input[6]),
            _mm256_alignr_epi8(input[7], input[2], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_7_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_blend_epi32(input[0], input[6], 0x33),
            _mm256_unpacklo_epi64(input[7], input[2]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_7_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[2], input[7]),
            _mm256_alignr_epi8(input[5], input[6], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_7_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[4], input[0]),
            _mm256_blend_epi32(input[4], input[3], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_7_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[5], input[3]),
            _mm256_shuffle_epi32(input[1], mm_shuffle(1, 0, 3, 2)),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_8_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[6], input[3]),
            _mm256_blend_epi32(input[1], input[6], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_8_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[7], input[5], 8),
            _mm256_unpackhi_epi64(input[0], input[4]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_8_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_blend_epi32(input[2], input[1], 0x33),
            _mm256_alignr_epi8(input[4], input[7], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_8_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[5], input[0]),
            _mm256_unpacklo_epi64(input[2], input[3]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_9_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[3], input[7]),
            _mm256_alignr_epi8(input[0], input[5], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_9_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[7], input[4]),
            _mm256_alignr_epi8(input[4], input[1], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_9_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[5], input[6]),
            _mm256_unpackhi_epi64(input[6], input[0]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_9_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_alignr_epi8(input[1], input[2], 8),
            _mm256_alignr_epi8(input[2], input[3], 8),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_10_1_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[5], input[4]),
            _mm256_unpackhi_epi64(input[3], input[0]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_10_1_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpacklo_epi64(input[1], input[2]),
            _mm256_blend_epi32(input[2], input[3], 0x33),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_10_2_1(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_unpackhi_epi64(input[6], input[7]),
            _mm256_unpackhi_epi64(input[4], input[1]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_10_2_2(input: &[__m256i; 8]) -> __m256i {
    unsafe {
        _mm256_blend_epi32(
            _mm256_blend_epi32(input[5], input[0], 0x33),
            _mm256_unpacklo_epi64(input[7], input[6]),
            0xF0,
        )
    }
}

#[inline(always)]
fn input_11_1_1(input: &[__m256i; 8]) -> __m256i {
    input_1_1_1(input)
}

#[inline(always)]
fn input_11_1_2(input: &[__m256i; 8]) -> __m256i {
    input_1_1_2(input)
}

#[inline(always)]
fn input_11_2_1(input: &[__m256i; 8]) -> __m256i {
    input_1_2_1(input)
}

#[inline(always)]
fn input_11_2_2(input: &[__m256i; 8]) -> __m256i {
    input_1_2_2(input)
}

#[inline(always)]
fn input_12_1_1(input: &[__m256i; 8]) -> __m256i {
    input_2_1_1(input)
}

#[inline(always)]
fn input_12_1_2(input: &[__m256i; 8]) -> __m256i {
    input_2_1_2(input)
}

#[inline(always)]
fn input_12_2_1(input: &[__m256i; 8]) -> __m256i {
    input_2_2_1(input)
}

#[inline(always)]
fn input_12_2_2(input: &[__m256i; 8]) -> __m256i {
    input_2_2_2(input)
}

#[inline(always)]
fn load16x64(state: &[u64; 16]) -> (__m256i, __m256i, __m256i, __m256i) {
    let ptr = state.as_ptr() as *const __m256i;
    unsafe {
        (
            _mm256_loadu_si256(ptr),
            _mm256_loadu_si256(ptr.add(1)),
            _mm256_loadu_si256(ptr.add(2)),
            _mm256_loadu_si256(ptr.add(3)),
        )
    }
}

#[inline(always)]
fn store8x64(output: &mut [u64; 8], a: __m256i, b: __m256i) {
    let ptr = output.as_mut_ptr() as *mut __m256i;
    unsafe {
        _mm256_storeu_si256(ptr, a);
        _mm256_storeu_si256(ptr.add(1), b);
    }
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub(super) fn compress(output: &mut [u64; 8], state: &mut [u64; 16], input: &[u64; 16]) {
    let (a, b, c, d) = load16x64(&*state);
    let (a0, b0) = (a, b);
    let input = broadcast_input(input);

    let x = input_1_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_1_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_1_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_1_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_2_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_2_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_2_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_2_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_3_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_3_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_3_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_3_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_4_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_4_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_4_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_4_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_5_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_5_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_5_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_5_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_6_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_6_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_6_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_6_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_7_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_7_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_7_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_7_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_8_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_8_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_8_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_8_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_9_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_9_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_9_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_9_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_10_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_10_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_10_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_10_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_11_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_11_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_11_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_11_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let x = input_12_1_1(&input);
    let (a, b, c, d) = r_1_1(a, b, c, d, x);
    let y = input_12_1_2(&input);
    let (a, b, c, d) = r_1_2(a, b, c, d, y);
    let x = input_12_2_1(&input);
    let (a, b, c, d) = r_2_1(a, b, c, d, x);
    let y = input_12_2_2(&input);
    let (a, b, c, d) = r_2_2(a, b, c, d, y);

    let (a, b) = unsafe {
        (
            _mm256_xor_si256(_mm256_xor_si256(a, c), a0),
            _mm256_xor_si256(_mm256_xor_si256(b, d), b0),
        )
    };
    store8x64(output, a, b);
}
