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

use crate::circuit::circuitbuilder::{Constant, LinearCombination, Scope};
use crate::ring::UnitalRing;
use core::array;

pub trait Convolution<R: UnitalRing, const N: usize>: Copy + Eq + Send + Sync {
    fn convolute(
        scope: &Scope<R>,
        a: [LinearCombination<R>; N],
        b: [LinearCombination<R>; N],
    ) -> [LinearCombination<R>; N];
}

pub fn binomial<R: UnitalRing, const N: usize>(
    scope: &Scope<R>,
    c: &mut [LinearCombination<R>],
    a: &[LinearCombination<R>],
    b: &[LinearCombination<R>],
    zeta: Constant<R>,
) {
    //TODO Karatsuba method
    let mut ab: [[LinearCombination<R>; N]; N] =
        array::from_fn(|_| array::from_fn(|_| Default::default()));
    for i in 0..N {
        for j in 0..N {
            let t = scope.auxiliary();
            scope.constrain(&a[i] * &b[j], t);
            ab[i][j] = t.into();
        }
    }
    match N {
        4 => {
            c[0] = &ab[0][0] - zeta * (&ab[1][3] + &ab[2][2] + &ab[3][1]);
            c[1] = &ab[0][1] + &ab[1][0] - zeta * (&ab[2][3] + &ab[3][2]);
            c[2] = &ab[0][2] + &ab[1][1] + &ab[2][0] - zeta * (&ab[3][3]);
            c[3] = &ab[0][3] + &ab[1][2] + &ab[2][1] + &ab[3][0];
        }
        3 => {
            c[0] = &ab[0][0] - zeta * (&ab[1][2] + &ab[2][1]);
            c[1] = &ab[0][1] + &ab[1][0] - zeta * (&ab[2][2]);
            c[2] = &ab[0][2] + &ab[1][1] + &ab[2][0];
        }
        2 => {
            c[0] = &ab[0][0] - zeta * (&ab[1][1]);
            c[1] = &ab[0][1] + &ab[1][0];
        }
        _ => {
            unimplemented!("Binomial convolution of length = {N}");
        }
    }
}
