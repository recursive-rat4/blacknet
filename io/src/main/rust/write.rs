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

use crate::error::Result;
use alloc::vec::Vec;

pub trait Write {
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
}

impl Write for Vec<u8> {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend(buf);
        Ok(())
    }
}

impl<W: Write> Write for &mut W {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        (*self).write_all(buf)
    }
}
