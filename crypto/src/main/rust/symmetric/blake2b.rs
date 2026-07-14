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

//! BLAKE2b cryptographic hash function.
//!
//! <https://www.blake2.net/blake2.pdf>
//! <https://www.blake2.net/blake2x.pdf>

use crate::random::UniformGenerator;
use crate::symmetric::CompressionFunction;
use core::cmp::min;
use core::mem::transmute;
use zeroize::Zeroize;

type Word = u64;
const BLOCK_SIZE: usize = 128;
const STATE_LEN: usize = 8;
const IV: [Word; STATE_LEN] = [
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];

#[derive(Clone, Copy, Zeroize)]
pub struct Blake2b<const BYTES: usize> {
    state: [Word; STATE_LEN],
    counter: u128,
    buffer: [u8; BLOCK_SIZE],
    position: usize,
}

impl<const BYTES: usize> Blake2b<BYTES> {
    /// Construct new hasher.
    pub const fn new() -> Self {
        Self::with_params(1, 1, 0, 0, 0, 0, [0; 16])
    }

    /// Construct new hasher with personalization.
    ///
    /// All zeroes is identical to the standard output.
    pub const fn with_personalization(personalization: [u8; 16]) -> Self {
        Self::with_params(1, 1, 0, 0, 0, 0, personalization)
    }

    const fn with_params(
        fanout: u8,
        depth: u8,
        leaf_length: u32,
        node_offset: u32,
        xof_length: u32,
        inner_length: u8,
        personalization: [u8; 16],
    ) -> Self {
        const {
            assert!(BYTES > 0 && BYTES <= 64);
        }
        let mut personalization: [Word; 2] = unsafe { transmute(personalization) };
        let mut i = 0;
        while i < 2 {
            personalization[i] = personalization[i].to_le();
            i += 1;
        }

        let mut state = IV;
        state[0] ^= BYTES as Word;
        state[0] ^= (fanout as Word) << 16;
        state[0] ^= (depth as Word) << 24;
        state[0] ^= (leaf_length as Word) << 32;
        state[1] ^= node_offset as Word;
        state[1] ^= (xof_length as Word) << 32;
        state[2] ^= (inner_length as Word) << 8;
        state[6] ^= personalization[0];
        state[7] ^= personalization[1];

        Self {
            state,
            counter: 0,
            buffer: [0; BLOCK_SIZE],
            position: 0,
        }
    }

    #[inline(always)]
    const fn g(
        state: &mut [Word; STATE_LEN * 2],
        input: &[Word; STATE_LEN * 2],
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
        state: &mut [Word; STATE_LEN * 2],
        input: &[Word; STATE_LEN * 2],
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
        Self::g(state, input, 0, 4, 8, 12, s0, s1);
        Self::g(state, input, 1, 5, 9, 13, s2, s3);
        Self::g(state, input, 2, 6, 10, 14, s4, s5);
        Self::g(state, input, 3, 7, 11, 15, s6, s7);

        Self::g(state, input, 0, 5, 10, 15, s8, s9);
        Self::g(state, input, 1, 6, 11, 12, s10, s11);
        Self::g(state, input, 2, 7, 8, 13, s12, s13);
        Self::g(state, input, 3, 4, 9, 14, s14, s15);
    }

    fn compress(&mut self, finalize: bool) {
        let mut state = [0; STATE_LEN * 2];
        state[..STATE_LEN].copy_from_slice(&self.state);
        state[STATE_LEN..].copy_from_slice(&IV);
        state[12] ^= self.counter as Word;
        state[13] ^= (self.counter >> Word::BITS) as Word;
        if finalize {
            state[14] = !state[14];
        }

        let mut input: [Word; STATE_LEN * 2] = unsafe { transmute(self.buffer) };
        input = input.map(Word::from_le);

        Self::r(
            &mut state, &input, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        );
        Self::r(
            &mut state, &input, 14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3,
        );
        Self::r(
            &mut state, &input, 11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4,
        );
        Self::r(
            &mut state, &input, 7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8,
        );
        Self::r(
            &mut state, &input, 9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13,
        );
        Self::r(
            &mut state, &input, 2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9,
        );
        Self::r(
            &mut state, &input, 12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11,
        );
        Self::r(
            &mut state, &input, 13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10,
        );
        Self::r(
            &mut state, &input, 6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5,
        );
        Self::r(
            &mut state, &input, 10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0,
        );
        Self::r(
            &mut state, &input, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        );
        Self::r(
            &mut state, &input, 14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3,
        );

        for i in 0..STATE_LEN {
            self.state[i] ^= state[i] ^ state[i + STATE_LEN];
        }
    }

    fn update_impl(&mut self, input: &[u8]) {
        let mut offset: usize = 0;
        let mut remain: usize = input.len();
        while remain != 0 {
            if self.position == BLOCK_SIZE {
                self.counter += self.position as u128;
                self.compress(false);
                self.position = 0;
            }
            let process = min(remain, BLOCK_SIZE - self.position);
            self.buffer[self.position..self.position + process]
                .copy_from_slice(&input[offset..offset + process]);
            remain -= process;
            offset += process;
            self.position += process;
        }
    }

    /// Process input data.
    #[inline]
    pub fn update(&mut self, input: impl AsRef<[u8]>) {
        self.update_impl(input.as_ref())
    }

    /// Hash.
    pub fn finalize(mut self) -> [u8; BYTES] {
        self.counter += self.position as u128;
        self.buffer[self.position..].fill(0);
        self.compress(true);
        let state = self.state.map(Word::to_le_bytes);
        let state: [u8; size_of::<Word>() * STATE_LEN] = unsafe { transmute(state) };
        let mut hash = [0u8; BYTES];
        hash.copy_from_slice(&state[..BYTES]);
        hash
    }

    /// Hash a single message.
    pub fn digest(message: impl AsRef<[u8]>) -> [u8; BYTES] {
        let mut hasher = Self::new();
        hasher.update(message);
        hasher.finalize()
    }
}

impl<const BYTES: usize> Default for Blake2b<BYTES> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BYTES: usize> CompressionFunction for Blake2b<BYTES> {
    type Hash = [u8; BYTES];

    fn compress(a: &Self::Hash, b: &Self::Hash) -> Self::Hash {
        let mut hasher = Self::with_personalization(*b"2to1 compression");
        hasher.update(a);
        hasher.update(b);
        hasher.finalize()
    }
}

/// BLAKE2b-256
pub type Blake2b256 = Blake2b<32>;
/// BLAKE2b-512
pub type Blake2b512 = Blake2b<64>;

/// BLAKE2Xb extensible-output function.
#[derive(Clone, Copy, Zeroize)]
pub struct Blake2xb {
    state: Blake2b<64>,
    xof_length: u32,
}

impl Blake2xb {
    /// Construct new hasher.
    ///
    /// The digest length is set to the maximum.
    pub const fn new() -> Self {
        Self::with_length(u32::MAX)
    }

    /// Construct new hasher with digest length.
    pub const fn with_length(xof_length: u32) -> Self {
        Self {
            state: Blake2b::with_params(1, 1, 0, 0, xof_length, 0, [0; 16]),
            xof_length,
        }
    }

    /// Process input data.
    #[inline]
    pub fn update(&mut self, input: impl AsRef<[u8]>) {
        self.state.update_impl(input.as_ref())
    }

    /// Produce XOF output.
    pub fn finalize(self) -> XOFOutput {
        XOFOutput {
            buffer: [0; 64],
            position: 64,
            h0: self.state.finalize(),
            node_offset: 0,
            xof_length: self.xof_length,
        }
    }
}

impl Default for Blake2xb {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Zeroize)]
pub struct XOFOutput {
    buffer: [u8; 64],
    position: usize,
    h0: [u8; 64],
    node_offset: u32,
    xof_length: u32,
}

impl UniformGenerator for XOFOutput {
    type Output = u8;

    fn generate(&mut self) -> Self::Output {
        if self.position == self.buffer.len() {
            let mut hasher = Blake2b::<64>::with_params(
                0,
                0,
                64,
                self.node_offset,
                self.xof_length,
                64,
                [0; 16],
            );
            hasher.update(self.h0);
            self.buffer = hasher.finalize();
            self.position = 0;
            self.node_offset += 1;
        }
        let b = self.buffer[self.position];
        self.position += 1;
        b
    }
}
