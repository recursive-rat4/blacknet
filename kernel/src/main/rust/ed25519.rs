/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use crate::blake2b::{Blake2b256, Blake2b512, Hash};
use crate::error::Error;
use alloc::vec::Vec;
use blacknet_crypto::{
    algebra::{IntegerRing, One},
    bigint::UInt256,
    ed25519::{Edwards25519GroupAffine, Edwards25519GroupExtended, Field25519, Scalar25519},
};
use core::array::TryFromSliceError;
use core::mem::transmute;
use digest::Digest;
use serde::{Deserialize, Serialize};

const BASE: Edwards25519GroupExtended = unsafe {
    Edwards25519GroupExtended::const_from_unchecked(
        Field25519::from_unchecked(UInt256::from_hex(
            "216936D3CD6E53FEC0A4E231FDD6DC5C692CC7609525A7B2C9562D608F25D51A",
        )),
        Field25519::from_unchecked(UInt256::from_hex(
            "6666666666666666666666666666666666666666666666666666666666666658",
        )),
        Field25519::ONE,
        Field25519::from_unchecked(UInt256::from_hex(
            "67875F0FD78B766566EA4E8E64ABE37D20F09F80775152F56DDE8AB3A5B7DDA3",
        )),
    )
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signature {
    r: [u8; 32],
    s: [u8; 32],
}

impl Signature {
    pub const fn raw_r(self) -> [u8; 32] {
        self.r
    }

    pub const fn raw_s(self) -> [u8; 32] {
        self.s
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = TryFromSliceError;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        let bytes = <[u8; 64]>::try_from(vec.as_slice())?;
        let rs: [[u8; 32]; 2] = unsafe { transmute(bytes) };
        Ok(Self { r: rs[0], s: rs[1] })
    }
}

pub type PublicKey = [u8; 32];

pub fn to_public_key(private_key: PrivateKey) -> PublicKey {
    let (scalar, _) = parse_private_key(private_key);
    let bits = scalar.canonical().bits::<{ Scalar25519::BITS as usize }>();
    let extended = BASE * bits;
    let affine: Edwards25519GroupAffine = extended.into();
    affine.encode()
}

pub type PrivateKey = [u8; 32];

const fn check_version(bytes: [u8; 32]) -> bool {
    bytes[0] & 0xF0 == 0x10
}

pub fn to_private_key(mnemonic: &str) -> Option<PrivateKey> {
    let hash: [u8; 32] = Blake2b256::digest(mnemonic).into();
    if check_version(hash) {
        Some(hash)
    } else {
        None
    }
}

pub fn sign(_hash: Hash, _private_key: PrivateKey) -> Signature {
    todo!();
}

pub fn verify(_signature: Signature, _hash: Hash, _public_key: PublicKey) -> Result<(), Error> {
    todo!();
}

fn parse_private_key(private_key: PrivateKey) -> (Scalar25519, [u8; 32]) {
    let mut hash: [u8; 64] = Blake2b512::digest(private_key).into();
    hash[0] &= 0xF8;
    hash[31] &= 0x7F;
    hash[31] |= 0x40;
    let hash: [[u8; 32]; 2] = unsafe { transmute(hash) };
    let integer = UInt256::from_le_bytes(hash[0]);
    let scalar = Scalar25519::new(integer);
    (scalar, hash[1])
}
