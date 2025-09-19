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

#![allow(clippy::manual_is_multiple_of)]

use crate::circuit::circuitbuilder::{Constant, LinearCombination};
use crate::numbertheoretictransform::Twiddles;
use crate::ring::UnitalRing;

pub fn cooley_tukey<Z: UnitalRing + Twiddles<M>, const M: usize, const N: usize>(
    a: &mut [LinearCombination<Z>; N],
) {
    let inertia: usize = const {
        assert!(N % M == 0);
        N / M
    };
    let mut i;
    let mut j = 0;
    let mut k = N / 2;
    while k >= inertia {
        let mut l = 0;
        while l < N {
            j += 1;
            let zeta = Z::TWIDDLES[j];
            i = l;
            while i < l + k {
                let t = &a[i + k] * Constant::from(zeta);
                a[i + k] = &a[i] - &t;
                a[i] += t;
                i += 1;
            }
            l = i + k;
        }
        k >>= 1;
    }
}

pub fn gentleman_sande<Z: UnitalRing + Twiddles<M>, const M: usize, const N: usize>(
    a: &mut [LinearCombination<Z>; N],
) {
    let inertia: usize = const {
        assert!(N % M == 0);
        N / M
    };
    let mut i;
    let mut j = M;
    let mut k = inertia;
    while k <= N / 2 {
        let mut l = 0;
        while l < N {
            j -= 1;
            let zeta = -Z::TWIDDLES[j];
            i = l;
            while i < l + k {
                let t = a[i].clone();
                a[i] += a[i + k].clone();
                a[i + k] = t - &a[i + k];
                a[i + k] *= Constant::from(zeta);
                i += 1;
            }
            l = i + k;
        }
        k <<= 1;
    }
    a.iter_mut().for_each(|i| *i *= Constant::from(Z::SCALE));
}
