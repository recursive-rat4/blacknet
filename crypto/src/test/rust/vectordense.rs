/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use blacknet_crypto::vectordense::VectorDense;

type R = blacknet_crypto::field25519::Field25519;

#[test]
fn add() {
    let a = VectorDense::<R>::from([0, 4, 2].map(R::from));
    let b = VectorDense::<R>::from([7, 3, 5].map(R::from));
    let c = VectorDense::<R>::from([7, 7, 7].map(R::from));
    assert_eq!(&a + &b, c);
    assert_eq!(&b + &a, c);
}

#[test]
fn mul() {
    let a = VectorDense::<R>::from([2, 2, 2].map(R::from));
    let b = VectorDense::<R>::from([1, 2, 4].map(R::from));
    let c = VectorDense::<R>::from([2, 4, 8].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&b * &a, c);
}

#[test]
fn scalar() {
    let a = VectorDense::<R>::from([4, 5, 6].map(R::from));
    let b = R::from(2);
    let c = VectorDense::<R>::from([8, 10, 12].map(R::from));
    assert_eq!(a * b, c);
    //assert_eq!(b * a, c);
}

#[test]
fn sub() {
    let a = VectorDense::<R>::from([8, 5, 1].map(R::from));
    let b = VectorDense::<R>::from([7, 3, 0].map(R::from));
    let c = VectorDense::<R>::from([1, 2, 1].map(R::from));
    assert_eq!(&a - &b, c);
    assert_eq!(a - b, c);
}

#[test]
fn neg() {
    let a = VectorDense::<R>::from([7, 0, -1].map(R::from));
    let b = VectorDense::<R>::from([-7, 0, 1].map(R::from));
    assert_eq!(-&a, b);
    assert_eq!(-(-&a), a);
}

#[test]
fn cat() {
    let a = VectorDense::<R>::from([0, 1].map(R::from));
    let b = VectorDense::<R>::from([2, 3, 4].map(R::from));
    let c = VectorDense::<R>::from([0, 1, 2, 3, 4].map(R::from));
    let d = VectorDense::<R>::from([2, 3, 4, 0, 1].map(R::from));
    assert_eq!(a.cat(&b), c);
    assert_eq!(b.cat(&a), d);
}

#[test]
fn dot() {
    let a = VectorDense::<R>::from([1, 3, -5].map(R::from));
    let b = VectorDense::<R>::from([4, -2, -1].map(R::from));
    let c = R::from(3);
    let d = R::from(35);
    assert_eq!(a.dot(&b), c);
    assert_eq!(b.dot(&a), c);
    assert_eq!(a.dot(&a), d);
}
