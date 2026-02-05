/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use core::cmp::min;
use core::mem::transmute;

pub type Word = u32;
pub const KEY_SIZE: usize = 32;
const IV_SIZE: usize = 12;
pub const BLOCK_SIZE: usize = 64;
pub const L: usize = 16;
const SIGMA: [Word; 4] = [0x61707865, 0x3320646E, 0x79622D32, 0x6B206574];

/// ChaCha stream cipher. <https://cr.yp.to/chacha/chacha-20080128.pdf>
pub struct ChaCha<const ROUNDS: usize> {
    input: [Word; L],
}

impl<const ROUNDS: usize> ChaCha<ROUNDS> {
    pub fn new(key: [u8; KEY_SIZE], iv: [u8; IV_SIZE]) -> Self {
        let mut chacha = Self { input: [0; L] };
        chacha.reset(key, iv);
        chacha
    }

    pub fn reset(&mut self, key: [u8; KEY_SIZE], iv: [u8; IV_SIZE]) {
        self.input[..SIGMA.len()].copy_from_slice(&SIGMA);
        let key: [Word; 8] = unsafe { transmute(key) };
        self.input[SIGMA.len()..12].copy_from_slice(&key.map(Word::from_le));
        self.input[12] = 0;
        let iv: [Word; 3] = unsafe { transmute(iv) };
        self.input[13..].copy_from_slice(&iv.map(Word::from_le));
    }

    pub const fn counter(&self) -> Word {
        self.input[12]
    }

    pub const fn seek(&mut self, counter: Word) {
        self.input[12] = counter
    }

    #[inline]
    pub fn encrypt(&mut self, cipher_text: &mut [u8], plain_text: &[u8]) {
        self.crypt(cipher_text, plain_text)
    }

    #[inline]
    pub fn decrypt(&mut self, plain_text: &mut [u8], cipher_text: &[u8]) {
        self.crypt(plain_text, cipher_text)
    }

    const fn quarter(state: &mut [Word; L], a: usize, b: usize, c: usize, d: usize) {
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(16);
        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(12);
        state[a] = state[a].wrapping_add(state[b]);
        state[d] ^= state[a];
        state[d] = state[d].rotate_left(8);
        state[c] = state[c].wrapping_add(state[d]);
        state[b] ^= state[c];
        state[b] = state[b].rotate_left(7);
    }

    pub fn keystream(&mut self, output: &mut [Word; L]) {
        const {
            assert!(ROUNDS & 1 == 0);
        };
        let mut state = self.input;
        for _ in 0..ROUNDS / 2 {
            Self::quarter(&mut state, 0, 4, 8, 12);
            Self::quarter(&mut state, 1, 5, 9, 13);
            Self::quarter(&mut state, 2, 6, 10, 14);
            Self::quarter(&mut state, 3, 7, 11, 15);

            Self::quarter(&mut state, 0, 5, 10, 15);
            Self::quarter(&mut state, 1, 6, 11, 12);
            Self::quarter(&mut state, 2, 7, 8, 13);
            Self::quarter(&mut state, 3, 4, 9, 14);
        }
        for i in 0..L {
            output[i] = state[i].wrapping_add(self.input[i]);
        }
        self.input[12] = self.input[12].wrapping_add(1);
    }

    fn crypt(&mut self, y: &mut [u8], x: &[u8]) {
        let mut offset: usize = 0;
        let mut remain: usize = x.len();
        let mut state = [0 as Word; L];
        while remain != 0 {
            self.keystream(&mut state);
            let process = min(remain, BLOCK_SIZE);
            let bytes: [u8; BLOCK_SIZE] = unsafe { transmute(state.map(Word::to_le_bytes)) };
            for i in 0..process {
                y[offset + i] = x[offset + i] ^ bytes[i]
            }
            remain -= process;
            offset += process;
        }
    }
}

pub type ChaCha20 = ChaCha<20>;
