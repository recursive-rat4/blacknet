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

use crate::ring::Ring;

#[rustfmt::skip]
pub trait Convolution<R: Ring, const N: usize>
    : 'static
    + Copy
    + Eq
{
    fn convolute(a: [R; N], b: [R; N]) -> [R; N];
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Negacyclic {}

impl<R: Ring, const N: usize> Convolution<R, N> for Negacyclic {
    fn convolute(a: [R; N], b: [R; N]) -> [R; N] {
        let mut c = [R::ZERO; N];
        for k in 0..N {
            for i in 0..k + 1 {
                c[k] += a[i] * b[k - i]
            }
            for i in k + 1..N {
                c[k] -= a[i] * b[k + N - i]
            }
        }
        c
    }
}

pub trait Binomial<R: Ring, const N: usize>: Convolution<R, N> {
    const ZETA: R;

    fn convolute(a: [R; N], b: [R; N]) -> [R; N] {
        let mut c = [R::ZERO; N];
        if N == 4 {
            c[0] = a[0] * b[0] - Self::ZETA * (a[1] * b[3] + a[2] * b[2] + a[3] * b[1]);
            c[1] = a[0] * b[1] + a[1] * b[0] - Self::ZETA * (a[2] * b[3] + a[3] * b[2]);
            c[2] = a[0] * b[2] + a[1] * b[1] + a[2] * b[0] - Self::ZETA * (a[3] * b[3]);
            c[3] = a[0] * b[3] + a[1] * b[2] + a[2] * b[1] + a[3] * b[0];
        } else if N == 3 {
            c[0] = a[0] * b[0] - Self::ZETA * (a[1] * b[2] + a[2] * b[1]);
            c[1] = a[0] * b[1] + a[1] * b[0] - Self::ZETA * (a[2] * b[2]);
            c[2] = a[0] * b[2] + a[1] * b[1] + a[2] * b[0];
        } else if N == 2 {
            c[0] = a[0] * b[0] - Self::ZETA * (a[1] * b[1]);
            c[1] = a[0] * b[1] + a[1] * b[0];
        } else {
            unimplemented!("Binomial convolution for N = {N}");
        }
        c
    }
}
