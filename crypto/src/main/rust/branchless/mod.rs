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

//! Utils for branchless algorithms.
//!
//! When the `cmov` feature is enabled the implementations delegate to the same named crate.
//! Otherwise implementations are in plain Rust.

mod absolute;
mod assign;
mod option;
mod order;
mod select;
mod swap;

pub use absolute::BlAbs;
pub use assign::BlAssign;
pub use option::BlOption;
pub use order::{BlEq, BlOrd};
pub use select::BlSelect;
pub use swap::BlSwap;
