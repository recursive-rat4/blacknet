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

//! BLAKE2b cryptographic hash function.
//!
//! <https://www.blake2.net/blake2.pdf>
//!
//! <https://www.blake2.net/blake2x.pdf>

cfg_select! {
    target_feature = "avx2" => {
        mod avx2;
        use avx2::compress;
    }
    _ => {
        mod generic;
        use generic::compress;
    }
}

mod blake2b;
mod blake2xb;

pub use blake2b::{Blake2b, Blake2b256, Blake2b512};
pub use blake2xb::{Blake2xb, XOFOutput};
