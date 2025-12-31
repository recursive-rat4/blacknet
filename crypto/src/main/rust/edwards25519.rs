/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use crate::algebra::{
    TwistedEdwardsGroupAffine, TwistedEdwardsGroupExtended, TwistedEdwardsGroupParams,
    TwistedEdwardsGroupProjective,
};
use crate::bigint::UInt256;
use crate::field25519::Field25519;

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
pub type Edwards25519GroupExtended = TwistedEdwardsGroupExtended<Edwards25519GroupParams>;
pub type Edwards25519GroupProjective = TwistedEdwardsGroupProjective<Edwards25519GroupParams>;
