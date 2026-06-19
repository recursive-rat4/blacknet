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

use crate::blake2b::{Blake2b256, Blake2b512, Digest, Hash};
use crate::error::Error;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use blacknet_crypto::{
    algebra::{IntegerRing, One},
    bigint::UInt256,
    ed25519::{Edwards25519GroupAffine, Edwards25519GroupExtended, Field25519, Scalar25519},
};
use core::array::TryFromSliceError;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::mem::transmute;
use data_encoding::{DecodeError, DecodeKind, HEXUPPER};
use serde::{Deserialize, Serialize};

// For compatibility, implementation follows eddsa-java 0.3.0
// https://eprint.iacr.org/2020/1244

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

#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signature {
    r: [u8; 32],
    s: [u8; 32],
}

impl Signature {
    pub const fn raw_r(&self) -> &[u8; 32] {
        &self.r
    }

    pub const fn raw_s(&self) -> &[u8; 32] {
        &self.s
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}{}",
            HEXUPPER.encode(&self.r),
            HEXUPPER.encode(&self.s)
        )
    }
}

impl TryFrom<&str> for Signature {
    type Error = DecodeError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if hex.len() == 128 {
            let (left, right) = hex.as_bytes().split_at(64);
            let mut r = [0_u8; 32];
            match HEXUPPER.decode_mut(left, &mut r) {
                Ok(_) => {}
                Err(err) => return Err(err.error),
            };
            let mut s = [0_u8; 32];
            match HEXUPPER.decode_mut(right, &mut s) {
                Ok(_) => {}
                Err(err) => return Err(err.error),
            };
            Ok(Self { r, s })
        } else {
            Err(DecodeError {
                position: 0,
                kind: DecodeKind::Length,
            })
        }
    }
}

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PublicKey([u8; 32]);

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", HEXUPPER.encode(&self.0))
    }
}

impl From<[u8; 32]> for PublicKey {
    fn from(array: [u8; 32]) -> Self {
        Self(array)
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = TryFromSliceError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(slice.try_into()?))
    }
}

impl TryFrom<Vec<u8>> for PublicKey {
    type Error = Vec<u8>;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(vec.try_into()?))
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = DecodeError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if hex.len() == 64 {
            let mut buf = [0_u8; 32];
            match HEXUPPER.decode_mut(hex.as_bytes(), &mut buf) {
                Ok(_) => Ok(Self(buf)),
                Err(err) => Err(err.error),
            }
        } else {
            Err(DecodeError {
                position: 0,
                kind: DecodeKind::Length,
            })
        }
    }
}

pub fn to_public_key(secret_key: SecretKey) -> PublicKey {
    let (scalar, _) = parse_secret_key(secret_key);
    let bytes = mul_base_encode(scalar);
    PublicKey(bytes)
}

#[derive(Clone, Copy, Default)]
pub struct SecretKey([u8; 32]);

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&str> for SecretKey {
    type Error = DecodeError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if hex.len() == 64 {
            let mut buf = [0_u8; 32];
            match HEXUPPER.decode_mut(hex.as_bytes(), &mut buf) {
                Ok(_) => Ok(Self(buf)),
                Err(err) => Err(err.error),
            }
        } else {
            Err(DecodeError {
                position: 0,
                kind: DecodeKind::Length,
            })
        }
    }
}

const fn check_version(bytes: [u8; 32]) -> bool {
    bytes[0] & 0xF0 == 0x10
}

pub fn to_secret_key(mnemonic: &str) -> Option<SecretKey> {
    let bytes: [u8; 32] = Blake2b256::digest(mnemonic).into();
    if check_version(bytes) {
        Some(SecretKey(bytes))
    } else {
        None
    }
}

pub fn sign(hash: Hash, secret_key: SecretKey) -> Signature {
    let (scalar, h) = parse_secret_key(secret_key);

    let mut hasher = Blake2b512::new();
    hasher.update(h);
    hasher.update(hash);
    let r: [u8; 64] = hasher.finalize().into();
    let r_scalar = Scalar25519::with_512(r);
    let r = mul_base_encode(r_scalar);

    let a = mul_base_encode(scalar);
    let mut hasher = Blake2b512::new();
    hasher.update(r);
    hasher.update(a);
    hasher.update(hash);
    let s: [u8; 64] = hasher.finalize().into();
    let s = Scalar25519::with_512(s);
    let s = r_scalar + s * scalar;
    let s = s.canonical().to_le_bytes();

    Signature { r, s }
}

pub fn verify(signature: Signature, hash: Hash, public_key: PublicKey) -> Result<(), Error> {
    let a = Edwards25519GroupAffine::decode(public_key.0)
        .ok_or_else(|| Error::Invalid("Invalid public key".to_owned()))?;
    let mut hasher = Blake2b512::new();
    hasher.update(signature.r);
    hasher.update(a.encode());
    hasher.update(hash);
    let h: [u8; 64] = hasher.finalize().into();
    let h = Scalar25519::with_512(h);
    let h = h.canonical().bits::<{ Scalar25519::BITS as usize }>();
    let a: Edwards25519GroupExtended = a.into();
    let s = UInt256::from_le_bytes(signature.s);
    let s = s
        .bits::<{ UInt256::BITS as usize }>()
        .into_iter()
        .take(s.bit_width() as usize);
    let r = BASE * s - a * h;
    let r: Edwards25519GroupAffine = r.into();
    if r.encode() == signature.r {
        Ok(())
    } else {
        Err(Error::Invalid("Invalid signature".to_owned()))
    }
}

fn parse_secret_key(secret_key: SecretKey) -> (Scalar25519, [u8; 32]) {
    let mut hash: [u8; 64] = Blake2b512::digest(secret_key).into();
    hash[0] &= 0xF8;
    hash[31] &= 0x7F;
    hash[31] |= 0x40;
    let hash: [[u8; 32]; 2] = unsafe { transmute(hash) };
    let integer = UInt256::from_le_bytes(hash[0]);
    let scalar = Scalar25519::new(integer);
    (scalar, hash[1])
}

fn mul_base_encode(scalar: Scalar25519) -> [u8; 32] {
    let bits = scalar.canonical().bits::<{ Scalar25519::BITS as usize }>();
    let extended = BASE.bl_mul(bits);
    let affine: Edwards25519GroupAffine = extended.into();
    affine.encode()
}
