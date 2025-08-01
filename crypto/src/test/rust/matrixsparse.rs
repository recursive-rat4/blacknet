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

use blacknet_crypto::matrixdense::MatrixDense;
use blacknet_crypto::matrixsparse::MatrixSparse;
use blacknet_crypto::vectordense::VectorDense;

type R = blacknet_crypto::field25519::Field25519;

#[test]
fn conversion() {
    let s = MatrixSparse::<R>::new(
        4,
        [0, 2, 5, 7, 9].into(),
        [0, 1, 1, 2, 3, 0, 3, 1, 3].into(),
        [1, 2, 3, 4, 5, 6, 7, 8, 9].map(R::from).into(),
    );
    #[rustfmt::skip]
    let d = MatrixDense::<R>::new(4, 4, [
        1, 2, 0, 0,
        0, 3, 4, 5,
        6, 0, 0, 7,
        0, 8, 0, 9,
    ].map(R::from).into());
    assert_eq!(MatrixSparse::from(&d), s);
    assert_eq!(MatrixDense::from(&s), d);
}

#[test]
fn vector() {
    let a = MatrixSparse::<R>::new(
        4,
        [0, 3, 3, 6, 9, 11].into(),
        [0, 1, 3, 0, 1, 3, 0, 1, 3, 1, 3].into(),
        [11, 12, 14, 31, 32, 34, 41, 42, 44, 52, 54]
            .map(R::from)
            .into(),
    );
    let b = VectorDense::<R>::from([61, 67, 71, 73].map(R::from));
    let c = VectorDense::<R>::from([2497, 0, 6517, 8527, 7426].map(R::from));
    assert_eq!(&a * &b, c);
}
