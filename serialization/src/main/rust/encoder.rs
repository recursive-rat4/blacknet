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

pub trait Encoder {
    fn encode_u8(&mut self, v: u8) -> Result<()>;
    fn encode_u16(&mut self, v: u16) -> Result<()>;
    fn encode_u32(&mut self, v: u32) -> Result<()>;
    fn encode_u64(&mut self, v: u64) -> Result<()>;

    fn encode_f32(&mut self, v: f32) -> Result<()>;
    fn encode_f64(&mut self, v: f64) -> Result<()>;

    fn encode_bytes(&mut self, v: &[u8]) -> Result<()>;

    fn encode_var_int(&mut self, v: u32) -> Result<()>;
}
