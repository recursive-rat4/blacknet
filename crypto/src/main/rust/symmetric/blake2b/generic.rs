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
const fn g(
    state: &mut [u64; 16],
    input: &[u64; 16],
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    x: usize,
    y: usize,
) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(input[x]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_right(32);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_right(24);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(input[y]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_right(63);
}

#[inline(always)]
const fn r(
    state: &mut [u64; 16],
    input: &[u64; 16],
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
    s12: usize,
    s13: usize,
    s14: usize,
    s15: usize,
) {
    g(state, input, 0, 4, 8, 12, s0, s1);
    g(state, input, 1, 5, 9, 13, s2, s3);
    g(state, input, 2, 6, 10, 14, s4, s5);
    g(state, input, 3, 7, 11, 15, s6, s7);

    g(state, input, 0, 5, 10, 15, s8, s9);
    g(state, input, 1, 6, 11, 12, s10, s11);
    g(state, input, 2, 7, 8, 13, s12, s13);
    g(state, input, 3, 4, 9, 14, s14, s15);
}

pub(super) fn compress(output: &mut [u64; 8], state: &mut [u64; 16], input: &[u64; 16]) {
    r(
        state, input, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    );
    r(
        state, input, 14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3,
    );
    r(
        state, input, 11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4,
    );
    r(
        state, input, 7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8,
    );
    r(
        state, input, 9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13,
    );
    r(
        state, input, 2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9,
    );
    r(
        state, input, 12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11,
    );
    r(
        state, input, 13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10,
    );
    r(
        state, input, 6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5,
    );
    r(
        state, input, 10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0,
    );
    r(
        state, input, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    );
    r(
        state, input, 14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3,
    );
    for i in 0..8 {
        output[i] ^= state[i] ^ state[i + 8];
    }
}
