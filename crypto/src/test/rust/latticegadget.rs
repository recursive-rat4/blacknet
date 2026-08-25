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

use blacknet_crypto::algebra::{Dot, IntegerModRing};
use blacknet_crypto::latticegadget;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector};

type Z = blacknet_crypto::pervushin::PervushinField;
type R = blacknet_crypto::pervushin::PervushinField2;

#[test]
#[rustfmt::skip]
fn matrix() {
    let params = latticegadget::Params::new(Z::from(65536), 65535, 16, 4);
    let a = DenseMatrix::new(2, 8, [
            3, 2, 1, 0,
            4, 2, 1, 0,
            5, 2, 1, 0,
            6, 2, 1, 0,
    ].map(Z::from).map(R::from).into());
    let b = DenseMatrix::new(2, 2, [
            R::from([4295098371, 0].map(Z::with_int)),
            R::from([4295098372, 0].map(Z::with_int)),
            R::from([4295098373, 0].map(Z::with_int)),
            R::from([4295098374, 0].map(Z::with_int)),
     ].into());
    let g = latticegadget::matrix::<Z, R>(2, 4, &params);
    assert_eq!(&a * &g.transpose(), b);
    let c = latticegadget::decompose_matrix(&b, &params);
    assert_eq!(c, a);
}

#[test]
fn vector() {
    let params = latticegadget::Params::new(Z::from(65536), 65535, 16, 4);
    let a = DenseVector::from([3, 2, 1, 0, 4, 2, 1, 0].map(Z::from).map(R::from));
    let b = DenseVector::from([4295098371, 4295098372].map(Z::with_int).map(R::from));
    let g = latticegadget::matrix::<Z, R>(2, 4, &params);
    assert_eq!(&g * &a, b);
    let c = latticegadget::decompose_vector(&b, &params);
    assert_eq!(c, a);
}

#[test]
fn polynomial() {
    let params = latticegadget::Params::new(Z::from(65536), 65535, 16, 4);
    let a = R::from([4444, 7789].map(Z::from));
    let b = R::from([34010, -59023].map(Z::from));
    let d = latticegadget::decompose_polynomial(&a, &params);
    let p = latticegadget::vector::<Z, R>(b, &params);
    assert_eq!(d.dot(p), a * b);
}

#[test]
fn integer() {
    let params = latticegadget::Params::new(Z::from(65536), 65535, 16, 4);
    let a = Z::from(78844277);
    let b = Z::from(-59023);
    let d = latticegadget::decompose_integer(&a, &params);
    let p = latticegadget::vector::<Z, Z>(b, &params);
    assert_eq!(d.dot(p), a * b);
}
