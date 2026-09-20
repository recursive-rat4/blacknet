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

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod decoder;
mod deserializer;
mod encoder;
mod error;
mod format;
mod reader;
mod serializer;
mod sizer;
mod writer;

pub use decoder::Decoder;
pub use deserializer::Deserializer;
pub use encoder::Encoder;
pub use error::{Error, Result};
pub use format::{from_bytes, from_read, to_bytes, to_size, to_write};
pub use reader::Reader;
pub use serializer::Serializer;
pub use sizer::Sizer;
pub use writer::Writer;
