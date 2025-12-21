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

use crate::error::{Error, Result};

pub trait Read {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;
}

impl Read for &[u8] {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        if let Some((right, left)) = self.split_at_checked(buf.len()) {
            buf.copy_from_slice(right);
            *self = left;
            Ok(())
        } else {
            Err(Error::unexpected_eof())
        }
    }
}

impl<R: Read> Read for &mut R {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        (*self).read_exact(buf)
    }
}
