/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::Zero;
use crate::assigner::assigment::Assigment;
use crate::gf2::GF2;
use core::array;

pub struct ArithmeticGate<'a> {
    assigment: &'a Assigment<GF2>,
}

impl<'a> ArithmeticGate<'a> {
    pub const fn new(assigment: &'a Assigment<GF2>) -> Self {
        Self { assigment }
    }

    pub fn wrapping_add<const N: usize>(&self, a: &[GF2; N], b: &[GF2; N]) -> [GF2; N] {
        let mut s = [GF2::ZERO; N];
        if N == 0 {
            return s;
        }
        let mut c = [GF2::ZERO; N];
        for i in 0..N - 1 {
            let acbc = (a[i] + c[i]) * (b[i] + c[i]);
            self.assigment.push(acbc);
            (s[i], c[i + 1]) = (a[i] + b[i] + c[i], c[i] + acbc);
        }
        s[N - 1] = a[N - 1] + b[N - 1] + c[N - 1];
        s
    }

    pub const fn rotate_right<const N: usize>(&self, a: &[GF2; N], n: u32) -> [GF2; N] {
        let n = n as usize % N;
        let (a_l, a_r) = a.split_at(n);
        let mut o = [GF2::ZERO; N];
        let (o_l, o_r) = o.split_at_mut(N - n);
        o_l.copy_from_slice(a_r);
        o_r.copy_from_slice(a_l);
        o
    }

    pub fn bitxor<const N: usize>(&self, a: &[GF2; N], b: &[GF2; N]) -> [GF2; N] {
        array::from_fn(|i| a[i] + b[i])
    }
}
