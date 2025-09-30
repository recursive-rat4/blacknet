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

pub trait Decoder {
    fn decode_u8(&mut self) -> Result<u8>;
    fn decode_u16(&mut self) -> Result<u16>;
    fn decode_u32(&mut self) -> Result<u32>;
    fn decode_u64(&mut self) -> Result<u64>;

    fn decode_f32(&mut self) -> Result<f32>;
    fn decode_f64(&mut self) -> Result<f64>;

    fn decode_bytes(&mut self, len: u32) -> Result<Vec<u8>>;

    fn decode_var_int(&mut self) -> Result<u32>;
}
