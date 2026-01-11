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

use blacknet_crypto::algebra::Tensor;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector};
use blacknet_crypto::norm::InfinityNorm;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn add() {
    let a = DenseVector::<R>::from([0, 4, 2].map(R::from));
    let b = DenseVector::<R>::from([7, 3, 5].map(R::from));
    let c = DenseVector::<R>::from([7, 7, 7].map(R::from));
    assert_eq!(&a + &b, c);
    assert_eq!(&b + &a, c);
}

#[test]
fn mul() {
    let a = DenseVector::<R>::from([2, 2, 2].map(R::from));
    let b = DenseVector::<R>::from([1, 2, 4].map(R::from));
    let c = DenseVector::<R>::from([2, 4, 8].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&b * &a, c);
}

#[test]
fn scalar() {
    let a = DenseVector::<R>::from([4, 5, 6].map(R::from));
    let b = R::from(2);
    let c = DenseVector::<R>::from([8, 10, 12].map(R::from));
    assert_eq!(a * b, c);
    //assert_eq!(b * a, c);
}

#[test]
fn sub() {
    let a = DenseVector::<R>::from([8, 5, 1].map(R::from));
    let b = DenseVector::<R>::from([7, 3, 0].map(R::from));
    let c = DenseVector::<R>::from([1, 2, 1].map(R::from));
    assert_eq!(&a - &b, c);
    assert_eq!(a - b, c);
}

#[test]
fn neg() {
    let a = DenseVector::<R>::from([7, 0, -1].map(R::from));
    let b = DenseVector::<R>::from([-7, 0, 1].map(R::from));
    assert_eq!(-&a, b);
    assert_eq!(-(-&a), a);
}

#[test]
fn cat() {
    let a = DenseVector::<R>::from([0, 1].map(R::from));
    let b = DenseVector::<R>::from([2, 3, 4].map(R::from));
    let c = DenseVector::<R>::from([0, 1, 2, 3, 4].map(R::from));
    let d = DenseVector::<R>::from([2, 3, 4, 0, 1].map(R::from));
    assert_eq!(a.cat(&b), c);
    assert_eq!(b.cat(&a), d);
}

#[test]
fn dot() {
    let a = DenseVector::<R>::from([1, 3, -5].map(R::from));
    let b = DenseVector::<R>::from([4, -2, -1].map(R::from));
    let c = R::from(3);
    let d = R::from(35);
    assert_eq!(a.dot(&b), c);
    assert_eq!(b.dot(&a), c);
    assert_eq!(a.dot(&a), d);
}

#[test]
#[rustfmt::skip]
fn module_tensor() {
    let a = DenseVector::<R>::from([
        0,
        1,
        2,
    ].map(R::from));
    let b = DenseVector::<R>::from([
        3,
        4,
    ].map(R::from));
    let c = DenseMatrix::<R>::new(3, 2, [
        0, 0,
        3, 4,
        6, 8,
    ].map(R::from).into());
    let d = DenseMatrix::<R>::new(2, 3, [
        0, 3, 6,
        0, 4, 8,
    ].map(R::from).into());
    assert_eq!((&a).tensor(&b), c);
    assert_eq!(b.tensor(a), d);
}

#[test]
fn kronecker_product() {
    let a = DenseVector::<R>::from([0, 1, 2].map(R::from));
    let b = DenseVector::<R>::from([3, 4].map(R::from));
    let c = DenseVector::<R>::from([0, 0, 3, 4, 6, 8].map(R::from));
    let d = DenseVector::<R>::from([0, 3, 6, 0, 4, 8].map(R::from));
    assert_eq!((&a).tensor(&b).vectorize(), c);
    assert_eq!(b.tensor(a).vectorize(), d);
}

#[test]
fn pad() {
    let a = DenseVector::<R>::from([1, 2, 3].map(R::from));
    let b = DenseVector::<R>::from([1, 2, 3, 0].map(R::from));
    assert_eq!(a.pad_to_power_of_two(), b);
    assert_eq!(b.pad_to_power_of_two(), b);
}

#[test]
fn infinity_norm() {
    let a = DenseVector::<R>::from([0, 1, 2, 3].map(R::from));
    let nb = 3;
    let ng = 4;
    assert!(!a.check_infinity_norm(&nb));
    assert!(a.check_infinity_norm(&ng));
}
