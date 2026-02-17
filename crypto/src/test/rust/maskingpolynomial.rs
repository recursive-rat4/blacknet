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

use blacknet_crypto::matrix::DenseVector;
use blacknet_crypto::polynomial::{
    Hypercube, InBasis, MaskingPolynomial, MultivariatePolynomial, Point,
};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn meta() {
    let mask = MaskingPolynomial::new([1, 2, 3, 4, 5].map(R::from).into(), 2, 2);
    assert_eq!(mask.degree(), 2);
    assert_eq!(mask.variables(), 2);
}

#[test]
fn sum() {
    let a = MaskingPolynomial::new([1, 2, 3, 4, 5, 6, 7].map(R::from).into(), 2, 3);
    let b = MaskingPolynomial::new([1].map(R::from).into(), 2, 0);
    let c = R::from(116);
    let d = R::from(1);
    assert_eq!(a.sum(), c);
    assert_eq!(b.sum(), d);
}

#[test]
fn bind() {
    let a = MaskingPolynomial::new([1, 2, 3, 4, 5].map(R::from).into(), 2, 2);
    let b = MaskingPolynomial::new([1, 2, 3].map(R::from).into(), 2, 1);
    let c = MaskingPolynomial::new([6].map(R::from).into(), 2, 0);

    let mut mask = a.clone();
    mask.bind(R::from(0));
    assert_eq!(mask, b);
    mask.bind(R::from(1));
    assert_eq!(mask, c);

    mask = a.clone();
    mask.bind(R::from(-2));
    assert_eq!(a.sum_with_var::<-2>(), Hypercube::<R>::sum(&mask));
    mask = a.clone();
    mask.bind(R::from(-1));
    assert_eq!(a.sum_with_var::<-1>(), Hypercube::<R>::sum(&mask));
    mask = a.clone();
    mask.bind(R::from(0));
    assert_eq!(a.sum_with_var::<0>(), Hypercube::<R>::sum(&mask));
    mask = a.clone();
    mask.bind(R::from(1));
    assert_eq!(a.sum_with_var::<1>(), Hypercube::<R>::sum(&mask));
    mask = a.clone();
    mask.bind(R::from(2));
    assert_eq!(a.sum_with_var::<2>(), Hypercube::<R>::sum(&mask));
    mask = a.clone();
    mask.bind(R::from(3));
    assert_eq!(a.sum_with_var::<3>(), Hypercube::<R>::sum(&mask));
    mask = a.clone();
    mask.bind(R::from(4));
    assert_eq!(a.sum_with_var::<4>(), Hypercube::<R>::sum(&mask));
}

#[test]
fn point() {
    let point = Point::<R>::from([7, 11].map(R::from));
    let mask = MaskingPolynomial::new([1, 2, 3, 4, 5].map(R::from).into(), 2, 2);
    let eval = R::from(659);
    assert_eq!(mask.point(&point), eval);
}

#[test]
fn basis() {
    let point = Point::<R>::from([7, 11].map(R::from));
    let mask = MaskingPolynomial::new([1, 2, 3, 4, 5].map(R::from).into(), 2, 2);
    let basis = DenseVector::from([1, 11, 121, 7, 49].map(R::from));
    assert_eq!(mask.basis(&point), basis);

    let point = Point::<R>::from([7].map(R::from));
    let mask = MaskingPolynomial::new([1, 2, 3].map(R::from).into(), 2, 1);
    let basis = DenseVector::from([1, 7, 49].map(R::from));
    assert_eq!(mask.basis(&point), basis);

    let point = Point::<R>::default();
    let mask = MaskingPolynomial::new([1].map(R::from).into(), 2, 0);
    let basis = DenseVector::from([1].map(R::from));
    assert_eq!(mask.basis(&point), basis);
}
