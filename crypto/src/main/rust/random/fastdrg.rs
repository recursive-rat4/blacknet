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

use crate::chacha::{BLOCK_SIZE, ChaCha, KEY_SIZE, L, Word};
use crate::random::UniformGenerator;
use core::mem::transmute;

pub const SEED_SIZE: usize = KEY_SIZE;

pub struct FastDRG {
    chacha: ChaCha<8>,
    buffer: [u8; BLOCK_SIZE],
    position: usize,
}

impl FastDRG {
    pub fn new(seed: [u8; SEED_SIZE]) -> Self {
        let mut chacha = ChaCha::<8>::new(seed, Default::default());
        let mut buffer = [0_u8; BLOCK_SIZE];
        Self::keystream(&mut chacha, &mut buffer);
        Self {
            chacha,
            buffer,
            position: 0,
        }
    }

    pub fn seed(&mut self, seed: [u8; SEED_SIZE]) {
        self.chacha.reset(seed, Default::default());
        Self::keystream(&mut self.chacha, &mut self.buffer);
        self.position = 0;
    }

    pub fn discard(&mut self, z: usize) {
        let pos_z = self.position + z;
        if pos_z <= BLOCK_SIZE {
            self.position = pos_z;
            return;
        }
        const {
            assert!(BLOCK_SIZE == 64);
        };
        let q = pos_z >> 6;
        let r = pos_z & 63;
        self.chacha.seek(self.chacha.counter() + q as u32 - 1);
        self.position = r;
        Self::keystream(&mut self.chacha, &mut self.buffer);
    }

    fn keystream(chacha: &mut ChaCha<8>, buffer: &mut [u8; BLOCK_SIZE]) {
        let mut scratch = [0 as Word; L];
        chacha.keystream(&mut scratch);
        let scratch: [u8; BLOCK_SIZE] = unsafe { transmute(scratch) };
        buffer.copy_from_slice(&scratch);
    }
}

impl Default for FastDRG {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl UniformGenerator for FastDRG {
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
}
