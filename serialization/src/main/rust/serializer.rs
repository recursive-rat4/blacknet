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
use crate::error::{Error, Result};
use serde::{Serialize, ser};

pub struct Serializer<E: Encoder> {
    encoder: E,
}

impl<E: Encoder> Serializer<E> {
    pub fn new(encoder: E) -> Self {
        Self { encoder }
    }

    pub fn encoder(self) -> E {
        self.encoder
    }
}

impl<E: Encoder> ser::Serializer for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_u8(v as u8)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_u16(v as u16)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_u32(v as u32)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.encoder.encode_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.encoder.encode_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.encoder.encode_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.encoder.encode_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.encoder.encode_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.encoder.encode_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        let mut buf = [0; 4];
        let bytes = v.encode_utf8(&mut buf).as_bytes();
        self.encoder.encode_bytes(bytes)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.encoder.encode_var_int(v.len() as u32)?;
        self.encoder.encode_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.encoder.encode_var_int(v.len() as u32)?;
        self.encoder.encode_bytes(v)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_u8(0)
    }

    fn serialize_some<S: ?Sized + Serialize>(self, value: &S) -> Result<()> {
        self.serialize_u8(1)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.encoder.encode_var_int(variant_index)
    }

    fn serialize_newtype_struct<S: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &S,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<S: ?Sized + Serialize>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &S,
    ) -> Result<()> {
        self.encoder.encode_var_int(variant_index)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(len) => {
                self.encoder.encode_var_int(len as u32)?;
                Ok(self)
            }
            None => Err(Error::Message("Unsized sequence".to_string())),
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.encoder.encode_var_int(variant_index)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.serialize_seq(len)?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.encoder.encode_var_int(variant_index)?;
        Ok(self)
    }
}

impl<E: Encoder> ser::SerializeSeq for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<E: Encoder> ser::SerializeTuple for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<E: Encoder> ser::SerializeTupleStruct for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<E: Encoder> ser::SerializeTupleVariant for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<E: Encoder> ser::SerializeMap for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<S: ?Sized + Serialize>(&mut self, key: &S) -> Result<()> {
        key.serialize(&mut **self)
    }

    fn serialize_value<S: ?Sized + Serialize>(&mut self, value: &S) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<E: Encoder> ser::SerializeStruct for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<S: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &S,
    ) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<E: Encoder> ser::SerializeStructVariant for &mut Serializer<E> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<S: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &S,
    ) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
