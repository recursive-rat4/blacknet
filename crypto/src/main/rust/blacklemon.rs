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

use crate::algebra::{IntegerRing, One, PolynomialRing, Zero};
use crate::lpr;
use crate::random::UniformGenerator;

// https://blacknet.ninja/blacklemon.pdf

const KAPPA: usize = 2;
#[expect(dead_code)]
const ELL: usize = lpr::D;
const R: <lpr::Zq as IntegerRing>::Int = 40;

pub struct SecretKey {
    a: lpr::SecretKey,
    b: lpr::RqNTT,
}

pub struct PublicKey {
    a: lpr::PublicKey,
    b: lpr::RqNTT,
}

pub type CipherText = lpr::CipherText;

pub type PlainText = lpr::PlainText;

pub fn generate_secret_key<RNG: UniformGenerator<Output = u8>>(rng: &mut RNG) -> SecretKey {
    SecretKey {
        a: lpr::generate_secret_key(rng),
        b: lpr::generate_uniform(rng),
    }
}

pub fn generate_public_key<RNG: UniformGenerator<Output = u8>>(
    rng: &mut RNG,
    sk: &SecretKey,
) -> PublicKey {
    PublicKey {
        a: lpr::generate_public_key(rng, &sk.a),
        b: -sk.b,
    }
}

pub fn encrypt<RNG: UniformGenerator<Output = u8>>(
    rng: &mut RNG,
    pk: &PublicKey,
    pt: &PlainText,
) -> CipherText {
    let mut ct = lpr::encrypt(rng, &pk.a, pt);
    ct.a += pk.b;
    ct
}

pub fn decrypt(sk: &SecretKey, ct: &CipherText) -> PlainText {
    let ct = CipherText {
        a: ct.a + sk.b,
        b: ct.b,
    };
    lpr::decrypt(&sk.a, &ct)
}

pub fn detect(sk: &SecretKey, ct: &CipherText) -> Option<PlainText> {
    let mut m = lpr::Rt::default();
    let d = ct.a + ct.b * sk.a.s + sk.b;
    let coefficients = d.coefficients();
    for i in 0..lpr::D {
        if coefficients[i].absolute() <= R {
            m[i] = lpr::Zt::ZERO;
        } else if lpr::DELTA - coefficients[i].absolute() <= R {
            m[i] = lpr::Zt::ONE;
        } else {
            return None;
        }
    }
    if m.coefficients()
        .into_iter()
        .take(KAPPA)
        .any(|coefficient| coefficient != lpr::Zt::ZERO)
    {
        return None;
    }
    Some(PlainText { m })
}
