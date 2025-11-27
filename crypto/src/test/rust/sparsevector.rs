/*
 * Copyright (c) 2025 Pavel Vasin
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

use blacknet_crypto::densematrix::DenseMatrix;
use blacknet_crypto::densevector::DenseVector;
use blacknet_crypto::sparsevector::SparseVector;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn conversion() {
    let s = SparseVector::<R>::new(8, [0, 2, 5, 7].into(), [1, 2, 3, 4].map(R::from).into());
    let d = DenseVector::<R>::from([1, 0, 2, 0, 0, 3, 0, 4].map(R::from));
    assert_eq!(SparseVector::from(&d), s);
    assert_eq!(DenseVector::from(&s), d);
}

#[test]
fn product() {
    #[rustfmt::skip]
    let a = DenseMatrix::<R>::new(2, 4, [
        11, 13, 17, 19,
        23, 29, 31, 37,
    ].map(R::from).into());
    let b = SparseVector::<R>::new(4, [1, 2].into(), [3, 5].map(R::from).into());
    let c = DenseVector::<R>::from([124, 242].map(R::from));
    let d = SparseVector::<R>::new(2, [0].into(), [3].map(R::from).into());
    let e = DenseVector::<R>::from([33, 39, 51, 57].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&d * &a, e);
}
