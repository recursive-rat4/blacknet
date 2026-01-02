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

use blacknet_crypto::algebra::AdditiveMonoid;
use blacknet_crypto::polynomial::{BinarityPolynomial, Hypercube, Polynomial};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn meta() {
    let bin = BinarityPolynomial::from([0, 0, 1, 0].map(R::from));
    assert_eq!(bin.degree(), 2);
    assert_eq!(bin.variables(), 2);
}

#[test]
fn point() {
    let f1 = BinarityPolynomial::from([1, -1].map(R::from));
    let f2 = BinarityPolynomial::from([2, -2].map(R::from));
    let f3 = BinarityPolynomial::from([1, 1, 0, 1].map(R::from));
    assert_ne!(Hypercube::sum(&f1), R::ZERO);
    assert_ne!(Hypercube::sum(&f2), R::ZERO);
    assert_eq!(Hypercube::sum(&f3), R::ZERO);
}
