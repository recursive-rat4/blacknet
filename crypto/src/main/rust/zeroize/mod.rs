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

//! Zeroization erases secret values from memory.
//!
//! Implementation relies on bytemuck traits to enable desirable optimizations,
//! and on inline assembly to prevent undesirable optimizations.

mod zeroize;
mod zeroizing;
mod zeroizingstring;

pub use zeroize::{zeroize, zeroize_slice, zeroize_string, zeroize_vec, zeroize_with_default};
pub use zeroizing::Zeroizing;
pub use zeroizingstring::ZeroizingString;
