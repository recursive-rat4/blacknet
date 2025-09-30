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
use alloc::string::String;
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};

pub struct Deserializer<D: Decoder> {
    decoder: D,
}

impl<D: Decoder> Deserializer<D> {
    pub fn new(decoder: D) -> Self {
        Self { decoder }
    }

    pub fn decoder(self) -> D {
        self.decoder
    }
}

impl<'de, D: Decoder> de::Deserializer<'de> for &mut Deserializer<D> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::StaticMessage("Unsupported"))
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.decoder.decode_u8()? {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            byte => Err(Error::InvalidBool(byte)),
        }
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(self.decoder.decode_u8()? as i8)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(self.decoder.decode_u16()? as i16)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.decoder.decode_u32()? as i32)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.decoder.decode_u64()? as i64)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.decoder.decode_u8()?)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.decoder.decode_u16()?)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.decoder.decode_u32()?)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(self.decoder.decode_u64()?)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.decoder.decode_f32()?)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.decoder.decode_f64()?)
    }

    fn deserialize_char<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        todo!("Utf-8 char");
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.decoder.decode_var_int()?;
        let bytes = self.decoder.decode_bytes(len)?;
        visitor.visit_string(String::from_utf8(bytes)?)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.decoder.decode_var_int()?;
        let bytes = self.decoder.decode_bytes(len)?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.decoder.decode_u8()? {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            byte => Err(Error::InvalidOption(byte)),
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.decoder.decode_var_int()?;
        visitor.visit_seq(Sequence::new(self, len))
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(Sequence::new(self, len as u32))
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.decoder.decode_var_int()?;
        visitor.visit_map(Sequence::new(self, len))
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        let len = fields.len() as u32;
        visitor.visit_seq(Sequence::new(self, len))
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_enum(Enum::new(self))
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let variant = self.decoder.decode_var_int()?;
        visitor.visit_u32(variant)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::StaticMessage("Unsupported"))
    }
}

struct Sequence<'a, D: Decoder> {
    de: &'a mut Deserializer<D>,
    len: u32,
    pos: u32,
}

impl<'a, D: Decoder> Sequence<'a, D> {
    fn new(de: &'a mut Deserializer<D>, len: u32) -> Self {
        Sequence { de, len, pos: 0 }
    }
}

impl<'de, 'a, D: Decoder> MapAccess<'de> for Sequence<'a, D> {
    type Error = Error;

    fn next_key_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        self.next_element_seed(seed)
    }

    fn next_value_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<T::Value> {
        seed.deserialize(&mut *self.de)
    }
}

impl<'de, 'a, D: Decoder> SeqAccess<'de> for Sequence<'a, D> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.pos != self.len {
            self.pos += 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct Enum<'a, D: Decoder> {
    de: &'a mut Deserializer<D>,
}

impl<'a, D: Decoder> Enum<'a, D> {
    fn new(de: &'a mut Deserializer<D>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a, D: Decoder> EnumAccess<'de> for Enum<'a, D> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let variant = seed.deserialize(&mut *self.de)?;
        Ok((variant, self))
    }
}

impl<'de, 'a, D: Decoder> VariantAccess<'de> for Enum<'a, D> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        let len = fields.len() as u32;
        visitor.visit_seq(Sequence::new(self.de, len))
    }
}
