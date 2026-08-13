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

use blacknet_crypto::polynomial::{UnivariatePolynomial, interpolation::*};

#[test]
fn balanced() {
    type Z = blacknet_crypto::pervushin::PervushinField;

    let p1 = UnivariatePolynomial::from([2, 3].map(Z::from));
    let p2 = UnivariatePolynomial::from([2, 3, 5].map(Z::from));
    let p3 = UnivariatePolynomial::from([2, 3, 5, 7].map(Z::from));
    let p4 = UnivariatePolynomial::from([2, 3, 5, 7, 11].map(Z::from));
    let p5 = UnivariatePolynomial::from([2, 3, 5, 7, 11, 13].map(Z::from));

    let interpolator = Interpolator::<Z>::degree_1().unwrap();
    assert_eq!(interpolator.interpolate(&[Z::from(2), Z::from(5)]), p1);

    let interpolator = Interpolator::<Z>::degree_2().unwrap();
    assert_eq!(
        interpolator.interpolate(&[Z::from(4), Z::from(2), Z::from(10)]),
        p2
    );

    let interpolator = Interpolator::<Z>::degree_3().unwrap();
    assert_eq!(
        interpolator.interpolate(&[Z::from(-3), Z::from(2), Z::from(17), Z::from(84)]),
        p3
    );

    let interpolator = Interpolator::<Z>::degree_4().unwrap();
    assert_eq!(
        interpolator.interpolate(&[
            Z::from(136),
            Z::from(8),
            Z::from(2),
            Z::from(28),
            Z::from(260)
        ]),
        p4
    );

    let interpolator = Interpolator::<Z>::degree_5().unwrap();
    assert_eq!(
        interpolator.interpolate(&[
            Z::from(-280),
            Z::from(-5),
            Z::from(2),
            Z::from(41),
            Z::from(676),
            Z::from(4295)
        ]),
        p5
    );
}

#[test]
fn characteristic_2() {
    type R = blacknet_crypto::gf2::RijndaelField;

    let xs = [0xC3, 0x9C, 0x15].map(R::new);
    let ys = [0x08, 0xFE, 0x47].map(R::new);
    let p = UnivariatePolynomial::from([0xD8, 0x1E, 0x52].map(R::new));

    let interpolator = Interpolator::<R>::with_xs(&xs).unwrap();
    assert_eq!(interpolator.interpolate(&ys), p);
}
