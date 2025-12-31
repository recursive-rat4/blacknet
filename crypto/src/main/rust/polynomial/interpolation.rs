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

use crate::algebra::UnitalAlgebra;
use crate::algebra::UnitalRing;
use crate::polynomial::UnivariatePolynomial;
use alloc::vec;

pub trait InterpolationConsts {
    const INV2: Self;
    const INV3: Self;
    const INV4: Self;
    const INV6: Self;
    const INV12: Self;
    const INV20: Self;
    const INV24: Self;
    const INV30: Self;
    const INV120: Self;
    const INV3_MUL2: Self;
    const INV4_MUL5: Self;
    const INV12_MUL5: Self;
    const INV12_MUL7: Self;
    const INV24_MUL7: Self;
}

pub fn interpolate_1<R: UnitalRing>(z0: R, p1: R) -> UnivariatePolynomial<R> {
    let a = p1 - z0;
    let b = z0;
    vec![b, a].into()
}

pub fn interpolate_2<R: UnitalRing + InterpolationConsts, A: UnitalAlgebra<R>>(
    n1: A,
    z0: A,
    p1: A,
) -> UnivariatePolynomial<A> {
    let a = p1 * R::INV2 + n1 * R::INV2 - z0;
    let b = p1 * R::INV2 - n1 * R::INV2;
    let c = z0;
    vec![c, b, a].into()
}

pub fn interpolate_3<R: UnitalRing + InterpolationConsts, A: UnitalAlgebra<R>>(
    n1: A,
    z0: A,
    p1: A,
    p2: A,
) -> UnivariatePolynomial<A> {
    let a = z0 * R::INV2 - p1 * R::INV2 + p2 * R::INV6 - n1 * R::INV6;
    let b = p1 * R::INV2 + n1 * R::INV2 - z0;
    let c = p1 - p2 * R::INV6 - n1 * R::INV3 - z0 * R::INV2;
    let d = z0;
    vec![d, c, b, a].into()
}

pub fn interpolate_4<R: UnitalRing + InterpolationConsts, A: UnitalAlgebra<R>>(
    n2: A,
    n1: A,
    z0: A,
    p1: A,
    p2: A,
) -> UnivariatePolynomial<A> {
    let a = z0 * R::INV4 - p1 * R::INV6 + p2 * R::INV24 - n1 * R::INV6 + n2 * R::INV24;
    let b = p2 * R::INV12 - p1 * R::INV6 + n1 * R::INV6 - n2 * R::INV12;
    let c =
        p1 * R::INV3_MUL2 - p2 * R::INV24 + n1 * R::INV3_MUL2 - n2 * R::INV24 - z0 * R::INV4_MUL5;
    let d = p1 * R::INV3_MUL2 - p2 * R::INV12 - n1 * R::INV3_MUL2 + n2 * R::INV12;
    let e = z0;
    vec![e, d, c, b, a].into()
}

pub fn interpolate_5<R: UnitalRing + InterpolationConsts, A: UnitalAlgebra<R>>(
    n2: A,
    n1: A,
    z0: A,
    p1: A,
    p2: A,
    p3: A,
) -> UnivariatePolynomial<A> {
    let a = p1 * R::INV12 - p2 * R::INV24 + p3 * R::INV120 + n1 * R::INV24
        - n2 * R::INV120
        - z0 * R::INV12;
    let b = z0 * R::INV4 - p1 * R::INV6 + p2 * R::INV24 - n1 * R::INV6 + n2 * R::INV24;
    let c = z0 * R::INV12_MUL5 - p1 * R::INV12_MUL7 + p2 * R::INV24_MUL7
        - p3 * R::INV24
        - n1 * R::INV24
        - n2 * R::INV24;
    let d =
        p1 * R::INV3_MUL2 - p2 * R::INV24 + n1 * R::INV3_MUL2 - n2 * R::INV24 - z0 * R::INV4_MUL5;
    let e = p1 - p2 * R::INV4 + p3 * R::INV30 - n1 * R::INV2 + n2 * R::INV20 - z0 * R::INV3;
    let f = z0;
    vec![f, e, d, c, b, a].into()
}
