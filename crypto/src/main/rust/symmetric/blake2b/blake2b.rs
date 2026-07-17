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

use crate::symmetric::CompressionFunction;
use crate::symmetric::blake2b::compress;
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

    pub(super) const fn with_params(
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

        compress(&mut self.state, &mut state, &input);
    }

    pub(super) fn update_impl(&mut self, input: &[u8]) {
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

/// Separated with personalization string `2to1 compression`.
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
