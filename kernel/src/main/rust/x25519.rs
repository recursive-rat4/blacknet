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

use crate::{
    ed25519::{PublicKey, SecretKey},
    error::Error,
};
use blacknet_crypto::{
    algebra::{IntegerModRing, One, Square, Zero},
    bigint::UInt256,
    branchless::BlSwap,
    ed25519::{Edwards25519Affine, Field25519},
    symmetric::{Blake2b256, Blake2b512},
    zeroize::zeroize,
};
use const_hex::FromHexError;
use core::fmt;

const A: Field25519 = unsafe {
    Field25519::from_unchecked(UInt256::from_hex(
        "000000000000000000000000000000000000000000000000000000000001DB42",
    ))
};

pub struct SharedKey([u8; 32]);

impl AsRef<[u8; 32]> for SharedKey {
    #[inline]
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for SharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedKey").finish_non_exhaustive()
    }
}

impl From<[u8; 32]> for SharedKey {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Drop for SharedKey {
    fn drop(&mut self) {
        zeroize(&mut self.0)
    }
}

impl TryFrom<&str> for SharedKey {
    type Error = FromHexError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        Ok(Self(const_hex::decode_to_array(hex)?))
    }
}

pub fn x25519(secret_key: &SecretKey, public_key: PublicKey) -> Result<SharedKey, Error> {
    let pub_key = parse_public_key(public_key)?;
    let sec_key = parse_secret_key(secret_key);

    let x1 = pub_key;
    let mut x2 = Field25519::ONE;
    let mut z2 = Field25519::ZERO;
    let mut x3 = pub_key;
    let mut z3 = Field25519::ONE;

    let mut swap = false;
    for i in (0..=254).rev() {
        let byte = unsafe { sec_key.get_unchecked(i >> 3) };
        let bit = ((byte >> (i & 7)) & 1) != 0;
        swap ^= bit;
        x2.bl_swap(&mut x3, swap);
        z2.bl_swap(&mut z3, swap);
        swap = bit;
        let mut tmp0 = x3 - z3;
        let mut tmp1 = x2 - z2;
        x2 += z2;
        z2 = x3 + z3;
        z3 = tmp0 * x2;
        z2 *= tmp1;
        tmp0 = tmp1.square();
        tmp1 = x2.square();
        x3 = z3 + z2;
        z2 = z3 - z2;
        x2 = tmp1 * tmp0;
        tmp1 -= tmp0;
        z2 = z2.square();
        z3 = tmp1 * A;
        x3 = x3.square();
        tmp0 += z3;
        z3 = x1 * z2;
        z2 = tmp1 * tmp0;
    }
    x2.bl_swap(&mut x3, swap);
    z2.bl_swap(&mut z3, swap);

    let (shared, is_ok) = (x2 / z2).into();
    if is_ok {
        let shared: [u8; 32] = shared.canonical().to_le_bytes();
        let hash = Blake2b256::digest(shared);
        Ok(SharedKey(hash))
    } else {
        Err(Error::invalid("Division by zero"))
    }
}

fn parse_public_key(public_key: PublicKey) -> Result<Field25519, Error> {
    let a = Edwards25519Affine::decode(public_key.into())
        .ok_or_else(|| Error::invalid("Invalid public key"))?;
    let (x, is_ok) = ((Field25519::ONE + a.y()) / (Field25519::ONE - a.y())).into();
    if is_ok {
        Ok(x)
    } else {
        Err(Error::invalid("Division by zero"))
    }
}

fn parse_secret_key(secret_key: &SecretKey) -> [u8; 32] {
    let mut hash: [u8; 64] = Blake2b512::digest(secret_key);
    hash[0] &= 0xF8;
    hash[31] &= 0x7F;
    hash[31] |= 0x40;
    hash.as_chunks::<32>().0[0]
}
