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

use crate::deserializer::Deserializer;
use crate::error::{Error, Result};
use crate::reader::Reader;
use crate::serializer::Serializer;
use crate::sizer::Sizer;
use crate::writer::Writer;
use serde::{Deserialize, Serialize};

pub fn from_bytes<'a, T: Deserialize<'a>>(bytes: &[u8], trail: bool) -> Result<T> {
    let reader = Reader::new(bytes);
    let mut deserializer = Deserializer::new(reader);
    let t = T::deserialize(&mut deserializer)?;
    let remaining = deserializer.decoder().input().len();
    if trail || remaining == 0 {
        Ok(t)
    } else {
        Err(Error::TrailingBytes(remaining))
    }
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let size = to_size(value)?;
    let writer = Writer::new(Vec::<u8>::with_capacity(size));
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(serializer.encoder().output())
}

pub fn to_size<T: Serialize>(value: &T) -> Result<usize> {
    let mut serializer = Serializer::new(Sizer::default());
    value.serialize(&mut serializer)?;
    Ok(serializer.encoder().output())
}
