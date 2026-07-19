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

    fn half_adder(&self, a: GF2, b: GF2) -> (GF2, GF2) {
        let ab = a * b;
        self.assigment.push(ab);

        let s = a + b;
        let c = ab;
        (s, c)
    }

    fn full_adder(&self, a: GF2, b: GF2, c: GF2) -> (GF2, GF2) {
        let ab = a * b;
        self.assigment.push(ab);
        let cab = c * (a + b);
        self.assigment.push(cab);

        let s = a + b + c;
        let c = ab + cab;
        (s, c)
    }

    pub fn wrapping_add<const N: usize>(&self, a: &[GF2; N], b: &[GF2; N]) -> [GF2; N] {
        let mut s = [GF2::ZERO; N];
        if N == 0 {
            return s;
        }
        // Ripple-carry adder
        let mut c: GF2;
        (s[0], c) = self.half_adder(a[0], b[0]);
        for i in 1..N {
            (s[i], c) = self.full_adder(a[i], b[i], c);
        }
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
