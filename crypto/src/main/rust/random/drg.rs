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

use crate::random::{BufferedGenerator, Seedable, UniformGenerator};
use crate::symmetric::chacha::{BLOCK_LEN, BLOCK_SIZE, ChaCha, KEY_SIZE};
use bytemuck::Zeroable;
use core::mem::transmute;

pub const SEED_SIZE: usize = KEY_SIZE;

#[derive(Clone, Copy, Zeroable)]
pub struct ChaChaDRG<const ROUNDS: usize> {
    chacha: ChaCha<ROUNDS>,
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
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const ROUNDS: usize> UniformGenerator for ChaChaDRG<ROUNDS> {
    type Output = [u8; BLOCK_SIZE];

    fn generate(&mut self) -> Self::Output {
        let mut buffer = [0; BLOCK_SIZE];
        Self::keystream(&mut self.chacha, &mut buffer);
        buffer
    }

    fn discard(&mut self, n: u32) {
        self.chacha.seek(self.chacha.counter() + n);
    }
}

impl<const ROUNDS: usize> Seedable for ChaChaDRG<ROUNDS> {
    type Seed = [u8; SEED_SIZE];

    fn from_seed(seed: &Self::Seed) -> Self {
        let chacha = ChaCha::<ROUNDS>::new(seed, &Default::default());
        Self { chacha }
    }

    fn reseed(&mut self, seed: &Self::Seed) {
        self.chacha.reset(seed, &Default::default());
    }
}

pub type FastDRG = BufferedGenerator<ChaChaDRG<8>, u8, BLOCK_SIZE>;
pub type StrongDRG = BufferedGenerator<ChaChaDRG<20>, u8, BLOCK_SIZE>;
