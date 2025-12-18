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

use crate::circuit::builder::{Constant, LinearCombination, Scope};
use crate::circuit::convolution::{Convolution, binomial};
use crate::numbertheoretictransform::Twiddles;
use core::array;

pub fn cooley_tukey<Z: Twiddles<M>, const M: usize, const N: usize>(
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
                let t = &a[i + k] * Constant::new(zeta);
                a[i + k] = &a[i] - &t;
                a[i] += t;
                i += 1;
            }
            l = i + k;
        }
        k >>= 1;
    }
}

pub fn gentleman_sande<Z: Twiddles<M>, const M: usize, const N: usize>(
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
                a[i + k] *= Constant::new(zeta);
                i += 1;
            }
            l = i + k;
        }
        k <<= 1;
    }
    a.iter_mut().for_each(|i| *i *= Constant::new(Z::SCALE));
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NTTConvolution<const M: usize, const N: usize> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Convolution<Z, N> for NTTConvolution<M, N> {
    fn convolute(
        scope: &Scope<Z>,
        a: [LinearCombination<Z>; N],
        b: [LinearCombination<Z>; N],
    ) -> [LinearCombination<Z>; N] {
        let inertia: usize = const {
            assert!(N % M == 0);
            N / M
        };
        match inertia {
            1 => array::from_fn(|i| {
                let c = scope.auxiliary();
                scope.constrain(&a[i] * &b[i], c);
                c.into()
            }),
            4 => {
                let k = inertia * 2;
                let l = N / k;
                let mut c: [LinearCombination<Z>; N] = array::from_fn(|_| Default::default());
                for i in 0..l {
                    binomial::<Z, 4>(
                        scope,
                        &mut c[i * k..i * k + 4],
                        &a[i * k..i * k + 4],
                        &b[i * k..i * k + 4],
                        Constant::new(-Z::TWIDDLES[l + i]),
                    );
                    binomial::<Z, 4>(
                        scope,
                        &mut c[i * k + inertia..i * k + inertia + 4],
                        &a[i * k + inertia..i * k + inertia + 4],
                        &b[i * k + inertia..i * k + inertia + 4],
                        Constant::new(Z::TWIDDLES[l + i]),
                    );
                }
                c
            }
            _ => {
                unimplemented!("NTT convolution with inertia = {inertia}");
            }
        }
    }
}
