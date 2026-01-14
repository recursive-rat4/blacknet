/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use crate::algebra::IntegerRing;
use crate::bigint::UInt256;
use crate::ed25519::field25519::Field25519;
use crate::ed25519::{
    TwistedEdwardsGroupAffine, TwistedEdwardsGroupExtended, TwistedEdwardsGroupParams,
    TwistedEdwardsGroupProjective,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

pub struct Edwards25519GroupParams;

impl TwistedEdwardsGroupParams for Edwards25519GroupParams {
    type F = Field25519;

    const A: Self::F = unsafe {
        Field25519::from_unchecked(UInt256::from_hex(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC7",
        ))
    };
    const D: Self::F = unsafe {
        Field25519::from_unchecked(UInt256::from_hex(
            "2C822B5A729FC526E5939207BC18869010A18777AFC6297380ED8BFEDF47E9FA",
        ))
    };

    const A_IS_MINUS_ONE: bool = true;
}

pub type Edwards25519GroupAffine = TwistedEdwardsGroupAffine<Edwards25519GroupParams>;

impl Serialize for Edwards25519GroupAffine {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (x, y) = (*self).into();
        let mut bytes: [u8; 32] = y.canonical().to_le_bytes();
        let x_is_odd = x.canonical().is_odd();
        bytes[31] |= (x_is_odd as u8) << 7;
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Edwards25519GroupAffine {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut bytes = <[u8; 32]>::deserialize(deserializer)?;
        let x_is_odd = (bytes[31] & 0x7F) != 0;
        bytes[31] &= 0x7F;
        let n = UInt256::from_le_bytes(bytes);
        let y = Field25519::new(n);
        Self::try_from_y(x_is_odd, y)
            .ok_or_else(|| D::Error::custom("Not a point on the elliptic curve"))
    }
}

pub type Edwards25519GroupExtended = TwistedEdwardsGroupExtended<Edwards25519GroupParams>;
pub type Edwards25519GroupProjective = TwistedEdwardsGroupProjective<Edwards25519GroupParams>;
