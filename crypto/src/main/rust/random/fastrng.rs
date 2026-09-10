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

use crate::random::{FastDRG, Seedable, drg::SEED_SIZE};
use blacknet_compat::getentropy;
use core::{
    cell::RefCell,
    mem::{MaybeUninit, transmute},
};
use std::thread_local;

/// # Panics
///
/// If initial entropy can't be obtained.
fn seed_fast() -> FastDRG {
    let mut seed = [MaybeUninit::<u8>::uninit(); SEED_SIZE];
    getentropy(&mut seed).expect("source of entropy");
    let seed: [u8; SEED_SIZE] = unsafe { transmute(seed) };
    FastDRG::from_seed(&seed)
}

thread_local! {
    pub static FAST_RNG: RefCell<FastDRG> = RefCell::new(seed_fast());
}
