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

use blacknet_crypto::algebra::IntegerRing;
use blacknet_crypto::latticegadget;
use blacknet_crypto::matrix::DenseVector;

type Z = blacknet_crypto::pervushin::PervushinField;
type R = blacknet_crypto::pervushin::PervushinField2;

#[test]
fn matrix() {
    let a = DenseVector::from([3, 2, 1, 0].map(Z::from).map(R::from));
    let b = DenseVector::from([4295098371].map(Z::new).map(R::from));
    let g = latticegadget::matrix(Z::from(65536), 1, 4);
    assert_eq!(&g * &a, b);
    let c = latticegadget::decompose_vector(&b, 65535, 16, 4);
    assert_eq!(c, a);
}

#[test]
fn vector() {
    let a = R::from([4444, 7789].map(Z::from));
    let b = R::from([34010, -59023].map(Z::from));
    let d = latticegadget::decompose(a, 65535, 16, 4);
    let p = latticegadget::vector(b, Z::from(65536), 4);
    assert_eq!(d.dot(&p), a * b);
}
