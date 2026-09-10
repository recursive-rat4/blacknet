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

use crate::random::Seedable;
use crate::zeroize::zeroize;
use blacknet_compat::getentropy;
use core::mem::{MaybeUninit, transmute};

/// # Panics
///
/// If initial entropy can't be obtained.
pub fn seed<G: Seedable<Seed = [u8; 32]>>() -> G {
    let mut seed = [MaybeUninit::<u8>::uninit(); 32];
    getentropy(&mut seed).expect("source of entropy");
    let mut seed: [u8; 32] = unsafe { transmute(seed) };
    let g = G::from_seed(&seed);
    zeroize(&mut seed);
    g
}
