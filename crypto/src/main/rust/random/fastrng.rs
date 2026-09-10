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

use crate::random::{FastDRG, Seedable, StrongDRG, UniformGenerator, drg::SEED_SIZE};
use crate::zeroize::zeroize;
use blacknet_compat::getentropy;
use core::{
    cell::RefCell,
    mem::{MaybeUninit, transmute},
};
use std::{
    sync::{LazyLock, Mutex},
    thread_local,
};

pub struct StrongSeeder {
    drg: StrongDRG,
}

impl StrongSeeder {
    /// # Panics
    ///
    /// If initial entropy can't be obtained.
    fn new() -> Self {
        let mut seed = [MaybeUninit::<u8>::uninit(); SEED_SIZE];
        getentropy(&mut seed).expect("source of entropy");
        let mut seed: [u8; SEED_SIZE] = unsafe { transmute(seed) };
        let drg = StrongDRG::from_seed(&seed);
        let seeder = Self { drg };
        zeroize(&mut seed);
        seeder
    }

    pub fn generate(&mut self, buf: &mut [MaybeUninit<u8>]) {
        for i in buf {
            i.write(self.drg.generate());
        }
    }
}

pub static STRONG_SEEDER: LazyLock<Mutex<StrongSeeder>> =
    LazyLock::new(|| Mutex::new(StrongSeeder::new()));

pub struct FastRNG {
    drg: FastDRG,
}

impl FastRNG {
    fn new() -> Self {
        let mut seed = [MaybeUninit::<u8>::uninit(); SEED_SIZE];
        STRONG_SEEDER.lock().unwrap().generate(&mut seed);
        let seed: [u8; SEED_SIZE] = unsafe { transmute(seed) };
        let drg = FastDRG::from_seed(&seed);
        Self { drg }
    }
}

impl UniformGenerator for FastRNG {
    type Output = <FastDRG as UniformGenerator>::Output;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.drg.generate()
    }

    #[inline]
    fn fill(&mut self, sequence: &mut [Self::Output]) {
        self.drg.fill(sequence)
    }

    #[inline]
    fn discard(&mut self, n: usize) {
        self.drg.discard(n)
    }
}

thread_local! {
    pub static FAST_RNG: RefCell<FastRNG> = RefCell::new(FastRNG::new());
}
