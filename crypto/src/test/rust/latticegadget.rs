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

use blacknet_crypto::algebra::IntegerModRing;
use blacknet_crypto::latticegadget::Gadget;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector, ScalarMatrix};

type Z = blacknet_crypto::pervushin::PervushinField;
type R = blacknet_crypto::pervushin::PervushinField2;

#[test]
#[rustfmt::skip]
fn matrix() {
    let gadget = Gadget::new(Z::from(65536), 65535, 16, 4);
    let a = DenseMatrix::new(2, 2, [
            R::from([4295098371, 0].map(Z::with_int)),
            R::from([4295098372, 0].map(Z::with_int)),
            R::from([4295098373, 0].map(Z::with_int)),
            R::from([4295098374, 0].map(Z::with_int)),
     ].into());
    let b = DenseMatrix::new(2, 8, [
            3, 2, 1, 0,
            4, 2, 1, 0,
            5, 2, 1, 0,
            6, 2, 1, 0,
    ].map(Z::from).map(R::from).into());
    let g = ScalarMatrix::new(2, DenseMatrix::new(4, 1,
        [1, 65536, 4294967296, 281474976710656]
        .map(Z::with_int).map(R::from).into()
    ));
    let c = gadget.compose_matrix::<R>(&b);
    let d = gadget.decompose_matrix(&a);
    let m = gadget.matrix::<R>(2, 4).transpose();
    assert_eq!(c, a);
    assert_eq!(d, b);
    assert_eq!(m, g);
}

#[test]
fn vector() {
    let gadget = Gadget::new(Z::from(65536), 65535, 16, 4);
    let a = DenseVector::from([4295098371, 4295098372].map(Z::with_int).map(R::from));
    let b = DenseVector::from([3, 2, 1, 0, 4, 2, 1, 0].map(Z::from).map(R::from));
    let g = DenseVector::from(
        [
            [3, 5].map(Z::with_int),
            [196608, 327680].map(Z::with_int),
            [12884901888, 21474836480].map(Z::with_int),
            [844424930131968, 1407374883553280].map(Z::with_int),
        ]
        .map(R::from),
    );
    let c = gadget.compose_vector::<R>(&b);
    let d = gadget.decompose_vector(&a);
    let v = gadget.vector::<R>(&R::from([3, 5].map(Z::from)));
    assert_eq!(c, a);
    assert_eq!(d, b);
    assert_eq!(v, g);
}

#[test]
fn polynomial() {
    let gadget = Gadget::new(Z::from(65536), 65535, 16, 4);
    let a = R::from([340102, -590231].map(Z::from));
    let b = DenseVector::from(
        [
            [12422, 65128].map(Z::from),
            [5, 65526].map(Z::from),
            [0, 65535].map(Z::from),
            [0, 8191].map(Z::from),
        ]
        .map(R::from),
    );
    let c = gadget.compose_polynomial::<R>(&b);
    let d = gadget.decompose_polynomial(&a);
    assert_eq!(c, a);
    assert_eq!(d, b);
}

#[test]
fn integer() {
    let gadget = Gadget::new(Z::from(65536), 65535, 16, 4);
    let a = Z::from(78844277);
    let b = DenseVector::from([4469, 1203, 0, 0].map(Z::from));
    let c = gadget.compose_integer(&b);
    let d = gadget.decompose_integer(&a);
    assert_eq!(c, a);
    assert_eq!(d, b);
}
