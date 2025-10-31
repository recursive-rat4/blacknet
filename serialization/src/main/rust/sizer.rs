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

#[derive(Default)]
pub struct Sizer {
    output: usize,
}

impl Sizer {
    pub const fn output(self) -> usize {
        self.output
    }
}

impl Encoder for Sizer {
    fn encode_u8(&mut self, _v: u8) -> Result<()> {
        self.output += 1;
        Ok(())
    }
    fn encode_u16(&mut self, _v: u16) -> Result<()> {
        self.output += 2;
        Ok(())
    }
    fn encode_u32(&mut self, _v: u32) -> Result<()> {
        self.output += 4;
        Ok(())
    }
    fn encode_u64(&mut self, _v: u64) -> Result<()> {
        self.output += 8;
        Ok(())
    }

    fn encode_f32(&mut self, _v: f32) -> Result<()> {
        self.output += 4;
        Ok(())
    }
    fn encode_f64(&mut self, _v: f64) -> Result<()> {
        self.output += 8;
        Ok(())
    }

    fn encode_bytes(&mut self, v: &[u8]) -> Result<()> {
        self.output += v.len();
        Ok(())
    }

    fn encode_var_int(&mut self, v: u32) -> Result<()> {
        let mut shift = 32 - v.leading_zeros();
        shift -= shift % 7;
        while shift != 0 {
            self.output += 1;
            shift -= 7;
        }
        self.output += 1;
        Ok(())
    }
}
