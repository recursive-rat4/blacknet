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

use crate::algebra::{IntegerRing, One, PolynomialRing, UnivariateRing, Zero};
use crate::convolution::Negacyclic;
use crate::fermat::{FermatField, FermatNTT1024, FermatRing1024};
use crate::integer::bits_u8;
use crate::random::{
    DiscreteGaussianDistribution, Distribution, UniformGenerator, UniformIntDistribution,
    fill_with_weight,
};
use crate::z2::Z2;
use core::array;

// https://eprint.iacr.org/2013/293

pub(crate) type Zt = Z2;
pub(crate) type Zq = FermatField;
pub(crate) const D: usize = 1024;
pub(crate) const H: usize = 64;
pub(crate) const SIGMA: f64 = 0.5;
pub(crate) type Rt = UnivariateRing<Zt, D, Negacyclic>;
#[expect(dead_code)]
pub(crate) type Rq = FermatRing1024;
pub(crate) type RqNTT = FermatNTT1024;
pub(crate) const DELTA: <Zq as IntegerRing>::Int = Zq::MODULUS >> 1;
pub(crate) const ZQ_DELTA: Zq = unsafe { Zq::from_unchecked(DELTA) };
pub(crate) const HALF_DELTA: <Zq as IntegerRing>::Int = Zq::MODULUS >> 2;

pub struct SecretKey {
    pub(crate) s: RqNTT,
}

pub struct PublicKey {
    pub(crate) a: RqNTT,
    pub(crate) b: RqNTT,
}

pub struct CipherText {
    pub(crate) a: RqNTT,
    pub(crate) b: RqNTT,
}

pub struct PlainText {
    pub(crate) m: Rt,
}

pub(crate) fn upscale(rt: &Rt) -> RqNTT {
    let coefficients = rt.coefficients();
    array::from_fn(|i| {
        if coefficients[i] == Zt::ZERO {
            Zq::ZERO
        } else {
            ZQ_DELTA
        }
    })
    .into()
}

pub(crate) fn generate_uniform<RNG: UniformGenerator<Output = u8>>(rng: &mut RNG) -> RqNTT {
    let mut uid = UniformIntDistribution::<<Zq as IntegerRing>::Int, RNG>::new(0..Zq::MODULUS);
    let residues: [<Zq as IntegerRing>::Int; D] = array::from_fn(|_| uid.sample(rng));
    let coefficients = residues.map(Zq::new);
    coefficients.into()
}

pub(crate) fn generate_error<RNG: UniformGenerator<Output = u8>>(rng: &mut RNG) -> RqNTT {
    let mut dgd = DiscreteGaussianDistribution::<i8, RNG>::new(0.0, SIGMA);
    let residues: [i8; D] = array::from_fn(|_| dgd.sample(rng));
    let coefficients = residues.map(Zq::from);
    coefficients.into()
}

pub fn generate_secret_key<RNG: UniformGenerator<Output = u8>>(rng: &mut RNG) -> SecretKey {
    let mut tud = UniformIntDistribution::<i8, RNG>::new(-1..=1);
    let mut residues = [0_i8; D];
    fill_with_weight(rng, &mut tud, &mut residues, H);
    let coefficients = residues.map(Zq::from);
    SecretKey {
        s: coefficients.into(),
    }
}

pub fn generate_public_key<RNG: UniformGenerator<Output = u8>>(
    rng: &mut RNG,
    sk: &SecretKey,
) -> PublicKey {
    let a = generate_uniform(rng);
    let e = generate_error(rng);
    PublicKey {
        a: -(a * sk.s + e),
        b: a,
    }
}

pub fn encrypt<RNG: UniformGenerator<Output = u8>>(
    rng: &mut RNG,
    pk: &PublicKey,
    pt: &PlainText,
) -> CipherText {
    let u = generate_secret_key(rng);
    let e1 = generate_error(rng);
    let e2 = generate_error(rng);
    CipherText {
        a: pk.a * u.s + e1 + upscale(&pt.m),
        b: pk.b * u.s + e2,
    }
}

pub fn decrypt(sk: &SecretKey, ct: &CipherText) -> PlainText {
    let d = ct.a + ct.b * sk.s;
    let coefficients = d.coefficients();
    let m: Rt = array::from_fn(|i| {
        if coefficients[i].absolute() <= HALF_DELTA {
            Zt::ZERO
        } else {
            Zt::ONE
        }
    })
    .into();
    PlainText { m }
}

pub fn encode(bytes: &[u8; D / 8]) -> PlainText {
    let mut m = Rt::ZERO;
    for (i, &byte) in bytes.iter().enumerate() {
        let bits = bits_u8::<8>(byte);
        for (j, bit) in bits.into_iter().enumerate() {
            if bit {
                m[i * 8 + j] = Zt::ONE;
            }
        }
    }
    PlainText { m }
}

pub fn decode(pt: &PlainText) -> [u8; D / 8] {
    let mut bytes = [0_u8; D / 8];
    for (i, chunk) in pt.m.coefficients().components().chunks_exact(8).enumerate() {
        let byte = chunk
            .iter()
            .rev()
            .map(|bit| if *bit == Zt::ZERO { 0 } else { 1 })
            .fold(0, |a, i| (a << 1) | i);
        bytes[i] = byte;
    }
    bytes
}
