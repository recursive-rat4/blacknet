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
    fn convolute(lps: [R; N], rps: [R; N]) -> [R; N];
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Negacyclic {}

impl<R: Ring, const N: usize> Convolution<R, N> for Negacyclic {
    fn convolute(lps: [R; N], rps: [R; N]) -> [R; N] {
        let mut s = [R::ZERO; N];
        for k in 0..N {
            for i in 0..k + 1 {
                s[k] += lps[i] * rps[k - i]
            }
            for i in k + 1..N {
                s[k] -= lps[i] * rps[k + N - i]
            }
        }
        s
    }
}
