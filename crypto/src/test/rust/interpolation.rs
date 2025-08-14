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

use blacknet_crypto::interpolation::*;
use blacknet_crypto::univariatepolynomial::UnivariatePolynomial;

type Z = blacknet_crypto::pervushin::PervushinField;

#[test]
fn balanced() {
    let p1 = UnivariatePolynomial::from([2, 3].map(Z::new));
    let p2 = UnivariatePolynomial::from([2, 3, 5].map(Z::new));
    let p3 = UnivariatePolynomial::from([2, 3, 5, 7].map(Z::new));
    let p4 = UnivariatePolynomial::from([2, 3, 5, 7, 11].map(Z::new));
    let p5 = UnivariatePolynomial::from([2, 3, 5, 7, 11, 13].map(Z::new));
    assert_eq!(interpolate_1(Z::new(2), Z::new(5)), p1);
    assert_eq!(interpolate_2(Z::new(4), Z::new(2), Z::new(10)), p2);
    assert_eq!(
        interpolate_3(Z::new(-3), Z::new(2), Z::new(17), Z::new(84)),
        p3
    );
    assert_eq!(
        interpolate_4(Z::new(136), Z::new(8), Z::new(2), Z::new(28), Z::new(260)),
        p4
    );
    assert_eq!(
        interpolate_5(
            Z::new(-280),
            Z::new(-5),
            Z::new(2),
            Z::new(41),
            Z::new(676),
            Z::new(4295)
        ),
        p5
    );
}
