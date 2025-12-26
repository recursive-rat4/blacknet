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

use blacknet_crypto::polynomial::{UnivariatePolynomial, interpolation::*};

type Z = blacknet_crypto::pervushin::PervushinField;

#[test]
fn balanced() {
    let p1 = UnivariatePolynomial::from([2, 3].map(Z::from));
    let p2 = UnivariatePolynomial::from([2, 3, 5].map(Z::from));
    let p3 = UnivariatePolynomial::from([2, 3, 5, 7].map(Z::from));
    let p4 = UnivariatePolynomial::from([2, 3, 5, 7, 11].map(Z::from));
    let p5 = UnivariatePolynomial::from([2, 3, 5, 7, 11, 13].map(Z::from));
    assert_eq!(interpolate_1(Z::from(2), Z::from(5)), p1);
    assert_eq!(interpolate_2(Z::from(4), Z::from(2), Z::from(10)), p2);
    assert_eq!(
        interpolate_3(Z::from(-3), Z::from(2), Z::from(17), Z::from(84)),
        p3
    );
    assert_eq!(
        interpolate_4(
            Z::from(136),
            Z::from(8),
            Z::from(2),
            Z::from(28),
            Z::from(260)
        ),
        p4
    );
    assert_eq!(
        interpolate_5(
            Z::from(-280),
            Z::from(-5),
            Z::from(2),
            Z::from(41),
            Z::from(676),
            Z::from(4295)
        ),
        p5
    );
}
