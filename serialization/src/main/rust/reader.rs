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

use crate::decoder::Decoder;
use crate::error::{Error, Result};
use alloc::vec;
use alloc::vec::Vec;
use blacknet_io::Read;

pub struct Reader<T: Read> {
    input: T,
}

impl<T: Read> Reader<T> {
    pub const fn new(input: T) -> Self {
        Self { input }
    }

    pub fn input(self) -> T {
        self.input
    }
}

impl<T: Read> Decoder for Reader<T> {
    fn decode_u8(&mut self) -> Result<u8> {
        let mut buf = [0_u8; 1];
        self.input.read_exact(&mut buf)?;
        Ok(u8::from_be_bytes(buf))
    }
    fn decode_u16(&mut self) -> Result<u16> {
        let mut buf = [0_u8; 2];
        self.input.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
    fn decode_u32(&mut self) -> Result<u32> {
        let mut buf = [0_u8; 4];
        self.input.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
    fn decode_u64(&mut self) -> Result<u64> {
        let mut buf = [0_u8; 8];
        self.input.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn decode_f32(&mut self) -> Result<f32> {
        let mut buf = [0_u8; 4];
        self.input.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }
    fn decode_f64(&mut self) -> Result<f64> {
        let mut buf = [0_u8; 8];
        self.input.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    fn decode_bytes(&mut self, len: u32) -> Result<Vec<u8>> {
        let mut buf = vec![0; len as usize];
        self.input.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn decode_var_int(&mut self) -> Result<u32> {
        let mut c = 5 + 1;
        let mut result: u32 = 0;
        let mut v: u8;
        loop {
            c -= 1;
            if c != 0 {
                v = self.decode_u8()?;
                result = result << 7 | v as u32 & 0x7F;
            } else {
                return Err(Error::TooLongVarInt);
            }
            if v & 0x80 != 0 {
                break;
            }
        }
        Ok(result)
    }
}
