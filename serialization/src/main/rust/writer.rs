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

use crate::encoder::Encoder;
use crate::error::Result;
use blacknet_io::Write;

pub struct Writer<T: Write> {
    output: T,
}

impl<T: Write> Writer<T> {
    pub fn new(output: T) -> Self {
        Self { output }
    }

    pub fn output(self) -> T {
        self.output
    }
}

impl<T: Write> Encoder for Writer<T> {
    fn encode_u8(&mut self, v: u8) -> Result<()> {
        let buf = v.to_be_bytes();
        self.output.write_all(&buf)?;
        Ok(())
    }
    fn encode_u16(&mut self, v: u16) -> Result<()> {
        let buf = v.to_be_bytes();
        self.output.write_all(&buf)?;
        Ok(())
    }
    fn encode_u32(&mut self, v: u32) -> Result<()> {
        let buf = v.to_be_bytes();
        self.output.write_all(&buf)?;
        Ok(())
    }
    fn encode_u64(&mut self, v: u64) -> Result<()> {
        let buf = v.to_be_bytes();
        self.output.write_all(&buf)?;
        Ok(())
    }

    fn encode_f32(&mut self, v: f32) -> Result<()> {
        let buf = v.to_be_bytes();
        self.output.write_all(&buf)?;
        Ok(())
    }
    fn encode_f64(&mut self, v: f64) -> Result<()> {
        let buf = v.to_be_bytes();
        self.output.write_all(&buf)?;
        Ok(())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<()> {
        self.output.write_all(v)?;
        Ok(())
    }

    fn encode_var_int(&mut self, v: u32) -> Result<()> {
        let mut shift = 32 - v.leading_zeros();
        shift -= shift % 7;
        while shift != 0 {
            self.encode_u8((v >> shift) as u8 & 0x7F)?;
            shift -= 7;
        }
        self.encode_u8(v as u8 & 0x7F | 0x80)?;
        Ok(())
    }
}
