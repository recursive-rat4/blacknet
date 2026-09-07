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

use crate::random::{Seedable, UniformGenerator};
use crate::symmetric::chacha::{BLOCK_LEN, BLOCK_SIZE, ChaCha, KEY_SIZE};
use core::mem::transmute;

pub const SEED_SIZE: usize = KEY_SIZE;

pub struct ChaChaDRG<const ROUNDS: usize> {
    chacha: ChaCha<ROUNDS>,
    buffer: [u8; BLOCK_SIZE],
    position: usize,
}

impl<const ROUNDS: usize> ChaChaDRG<ROUNDS> {
    pub fn new() -> Self {
        Self::from_seed(&Default::default())
    }

    fn keystream(chacha: &mut ChaCha<ROUNDS>, buffer: &mut [u8; BLOCK_SIZE]) {
        let mut scratch = [0u32; BLOCK_LEN];
        chacha.keystream(&mut scratch);
        let scratch: [u8; BLOCK_SIZE] = unsafe { transmute(scratch.map(u32::to_le_bytes)) };
        buffer.copy_from_slice(&scratch);
    }
}

impl<const ROUNDS: usize> Default for ChaChaDRG<ROUNDS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const ROUNDS: usize> UniformGenerator for ChaChaDRG<ROUNDS> {
    type Output = u8;

    fn generate(&mut self) -> Self::Output {
        if self.position != BLOCK_SIZE {
            let result = self.buffer[self.position];
            self.position += 1;
            result
        } else {
            self.position = 1;
            Self::keystream(&mut self.chacha, &mut self.buffer);
            self.buffer[0]
        }
    }

    fn discard(&mut self, n: usize) {
        let pos_n = self.position + n;
        if pos_n <= BLOCK_SIZE {
            self.position = pos_n;
            return;
        }
        const {
            assert!(BLOCK_SIZE == 64);
        };
        let q = pos_n >> 6;
        let r = pos_n & 63;
        self.chacha.seek(self.chacha.counter() + q as u32 - 1);
        self.position = r;
        Self::keystream(&mut self.chacha, &mut self.buffer);
    }
}

impl<const ROUNDS: usize> Seedable for ChaChaDRG<ROUNDS> {
    type Seed = [u8; SEED_SIZE];

    fn from_seed(seed: &Self::Seed) -> Self {
        let mut chacha = ChaCha::<ROUNDS>::new(seed, &Default::default());
        let mut buffer = [0u8; BLOCK_SIZE];
        Self::keystream(&mut chacha, &mut buffer);
        Self {
            chacha,
            buffer,
            position: 0,
        }
    }

    fn reseed(&mut self, seed: &Self::Seed) {
        self.chacha.reset(seed, &Default::default());
        Self::keystream(&mut self.chacha, &mut self.buffer);
        self.position = 0;
    }
}

pub type FastDRG = ChaChaDRG<8>;
pub type StrongDRG = ChaChaDRG<20>;
