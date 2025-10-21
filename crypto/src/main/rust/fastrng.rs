/*
 * Copyright (c) 2025 Pavel Vasin
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

#![cfg(feature = "std")]

use crate::distribution::{Distribution, UniformGenerator};
use crate::fastdrg::{FastDRG, SEED_SIZE};
use crate::uniformintdistribution::UniformIntDistribution;
use blacknet_compat::getentropy;
use core::array;
use core::cell::RefCell;
use std::sync::{LazyLock, Mutex};
use std::thread_local;

type Word = <FastDRG as UniformGenerator>::Output;

pub struct FastSeeder {
    drg: FastDRG,
}

impl FastSeeder {
    fn new() -> Self {
        let seed = getentropy::<SEED_SIZE>().expect("source of entropy");
        Self {
            drg: FastDRG::new(seed),
        }
    }

    pub fn generate<const N: usize>(&mut self) -> [u8; N] {
        let mut dst = UniformIntDistribution::<FastDRG>::new(0..256);
        array::from_fn(|_| dst.sample(&mut self.drg) as u8)
    }
}

pub static FAST_SEEDER: LazyLock<Mutex<FastSeeder>> =
    LazyLock::new(|| Mutex::new(FastSeeder::new()));

pub struct FastRNG {
    drg: FastDRG,
}

impl FastRNG {
    fn new() -> Self {
        let seed = FAST_SEEDER.lock().unwrap().generate::<SEED_SIZE>();
        Self {
            drg: FastDRG::new(seed),
        }
    }
}

impl UniformGenerator for FastRNG {
    type Output = Word;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.drg.generate()
    }
}

thread_local! {
    pub static FAST_RNG: RefCell<FastRNG> = RefCell::new(FastRNG::new());
}
