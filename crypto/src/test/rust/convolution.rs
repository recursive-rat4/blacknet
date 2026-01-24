/*
 * Copyright (c) 2026 Pavel Vasin
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

use blacknet_crypto::algebra::{IntegerRing, One};
use blacknet_crypto::convolution::*;

type Z = blacknet_crypto::fermat::FermatField;

struct TestBinomial;

impl Convolution<Z, 3> for TestBinomial {
    fn convolute(a: [Z; 3], b: [Z; 3]) -> [Z; 3] {
        <Self as Binomial<Z, 3>>::convolute(a, b)
    }
}

impl Binomial<Z, 3> for TestBinomial {
    const ZETA: Z = Z::ONE;
}

#[test]
fn cyclic() {
    let a = [3, 5, 7].map(Z::new);
    let b = [11, 13, 17].map(Z::new);
    let c = [209, 213, 193].map(Z::new);
    let d = Cyclic::convolute(a, b);
    assert_eq!(d, c);
}

#[test]
fn negacyclic() {
    let a = [3, 5, 7].map(Z::new);
    let b = [11, 13, 17].map(Z::new);
    let c = [-143, -25, 193].map(Z::new);
    let d = Negacyclic::convolute(a, b);
    assert_eq!(d, c);
}

#[test]
fn binomial() {
    let a = [3, 5, 7].map(Z::new);
    let b = [11, 13, 17].map(Z::new);
    let c = [209, 213, 193].map(Z::new);
    let d = <TestBinomial as Convolution<Z, 3>>::convolute(a, b);
    assert_eq!(d, c);
}
