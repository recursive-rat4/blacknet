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

use blacknet_crypto::freemodule::FreeModule;
use blacknet_crypto::hypercube::Hypercube;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector};
use blacknet_crypto::multilinearextension::MultilinearExtension;
use blacknet_crypto::operation::Double;
use blacknet_crypto::polynomial::Polynomial;
use core::iter::zip;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn meta() {
    let mle = MultilinearExtension::from([1, 2, 3, 4, 5, 6, 7, 8].map(R::from));
    assert_eq!(mle.degree(), 1);
    assert_eq!(mle.variables(), 3);
}

#[test]
fn add() {
    let a = MultilinearExtension::from([1, 2, 3, 4].map(R::from));
    let b = MultilinearExtension::from([5, 6, 1, 5].map(R::from));
    let c = MultilinearExtension::from([6, 8, 4, 9].map(R::from));
    assert_eq!(a + b, c);
}

#[test]
fn dbl() {
    let a = MultilinearExtension::from([1, 2, 3, 4].map(R::from));
    let b = MultilinearExtension::from([2, 4, 6, 8].map(R::from));
    assert_eq!(a.double(), b);
}

#[test]
fn sub() {
    let a = MultilinearExtension::from([99, 98, 97, 96].map(R::from));
    let b = R::from(3);
    let c = MultilinearExtension::from([96, 95, 94, 93].map(R::from));
    assert_eq!(a - b, c);
}

#[test]
fn neg() {
    let a = MultilinearExtension::from([7, 0, -1, 0].map(R::from));
    let b = MultilinearExtension::from([-7, 0, 1, 0].map(R::from));
    assert_eq!(-a, b);
}

#[test]
fn mul() {
    let a = MultilinearExtension::from([1, 2, 3, 4].map(R::from));
    let b = R::from(2);
    let c = MultilinearExtension::from([2, 4, 6, 8].map(R::from));
    assert_eq!(a * b, c);
}

#[test]
fn bind() {
    let a = MultilinearExtension::from([1, 2, 3, 4, 5, 6, 7, 8].map(R::from));
    let b = MultilinearExtension::from([1, 2, 3, 4].map(R::from));
    let c = MultilinearExtension::from([3, 4].map(R::from));
    let d = MultilinearExtension::from([4].map(R::from));

    let mut mle = a.clone();
    mle.bind(R::from(0));
    assert_eq!(mle, b);
    mle.bind(R::from(1));
    assert_eq!(mle, c);
    mle.bind(R::from(1));
    assert_eq!(mle, d);

    mle = a.clone();
    mle.bind(R::from(-2));
    let evaluations = a.hypercube_with_var::<-2>();
    assert_eq!(evaluations.elements(), mle.hypercube());
    mle = a.clone();
    mle.bind(R::from(-1));
    let evaluations = a.hypercube_with_var::<-1>();
    assert_eq!(evaluations.elements(), mle.hypercube());
    mle = a.clone();
    mle.bind(R::from(0));
    let evaluations = a.hypercube_with_var::<0>();
    assert_eq!(evaluations.elements(), mle.hypercube());
    mle = a.clone();
    mle.bind(R::from(1));
    let evaluations = a.hypercube_with_var::<1>();
    assert_eq!(evaluations.elements(), mle.hypercube());
    mle = a.clone();
    mle.bind(R::from(2));
    let evaluations = a.hypercube_with_var::<2>();
    assert_eq!(evaluations.elements(), mle.hypercube());
    mle = a.clone();
    mle.bind(R::from(3));
    let evaluations = a.hypercube_with_var::<3>();
    assert_eq!(evaluations.elements(), mle.hypercube());
    mle = a.clone();
    mle.bind(R::from(4));
    let evaluations = a.hypercube_with_var::<4>();
    assert_eq!(evaluations.elements(), mle.hypercube());
}

#[test]
fn matrix() {
    let hc = Hypercube::<R>::new(3);
    #[rustfmt::skip]
    let a = DenseMatrix::new(2, 4, [
        30, 31, 32, 33,
        43, 44, 45, 46,
    ].map(R::from).into());
    let mle = MultilinearExtension::from(a.clone());
    zip(hc.iter_rank2(2, 4), hc.iter_vertex())
        .for_each(|(index, b)| assert_eq!(mle.point(&b), a[index]));
}

#[test]
fn vector() {
    let hc = Hypercube::<R>::new(3);
    let a = DenseVector::from([63, 64, 65, 66, 67, 68, 69, 70].map(R::from));
    let mle = MultilinearExtension::from(a.clone());
    zip(hc.iter_index(), hc.iter_vertex())
        .for_each(|(index, b)| assert_eq!(mle.point(&b), a[index]));
}

#[test]
fn module() {
    let hc = Hypercube::<R>::new(2);
    let a = FreeModule::from([71, 72, 73, 74].map(R::from));
    let mle = MultilinearExtension::from_iter(a);
    zip(hc.iter_index(), hc.iter_vertex())
        .for_each(|(index, b)| assert_eq!(mle.point(&b), a[index]));
}
