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

use crate::convolution::{Convolution, binomial};
use crate::ring::UnitalRing;
use core::array;

// https://arxiv.org/abs/2211.13546

pub trait Twiddles<const N: usize>: UnitalRing {
    const TWIDDLES: [Self; N];
    const SCALE: Self;
}

pub fn cooley_tukey<Z: Twiddles<M>, const M: usize, const N: usize>(mut a: [Z; N]) -> [Z; N] {
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
                let t = a[i + k] * zeta;
                a[i + k] = a[i] - t;
                a[i] += t;
                i += 1;
            }
            l = i + k;
        }
        k >>= 1;
    }
    a
}

pub fn gentleman_sande<Z: Twiddles<M>, const M: usize, const N: usize>(mut a: [Z; N]) -> [Z; N] {
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
                let t = a[i];
                a[i] += a[i + k];
                a[i + k] = t - a[i + k];
                a[i + k] *= zeta;
                i += 1;
            }
            l = i + k;
        }
        k <<= 1;
    }
    a.map(|i| i * Z::SCALE)
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NTTConvolution<const M: usize, const N: usize> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Convolution<Z, N> for NTTConvolution<M, N> {
    fn convolute(a: [Z; N], b: [Z; N]) -> [Z; N] {
        let inertia: usize = const {
            assert!(N % M == 0);
            N / M
        };
        match inertia {
            1 => array::from_fn(|i| a[i] * b[i]),
            4 => {
                let k = inertia * 2;
                let l = N / k;
                let mut c = [Z::ZERO; N];
                for i in 0..l {
                    binomial::<Z, 4>(
                        &mut c[i * k..i * k + 4],
                        &a[i * k..i * k + 4],
                        &b[i * k..i * k + 4],
                        -Z::TWIDDLES[l + i],
                    );
                    binomial::<Z, 4>(
                        &mut c[i * k + inertia..i * k + inertia + 4],
                        &a[i * k + inertia..i * k + inertia + 4],
                        &b[i * k + inertia..i * k + inertia + 4],
                        Z::TWIDDLES[l + i],
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
