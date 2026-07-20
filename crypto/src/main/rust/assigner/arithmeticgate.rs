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

use crate::algebra::{Algebra, AlgebraOps};
use crate::assigner::assigment::Assigment;
use crate::gf2::GF2;
use core::array;

pub struct ArithmeticGate<'a, A: Algebra<GF2>> {
    assigment: &'a Assigment<A>,
}

impl<'a, A: Algebra<GF2> + Clone> ArithmeticGate<'a, A>
where
    for<'b> &'b A: AlgebraOps<GF2, A>,
{
    pub const fn new(assigment: &'a Assigment<A>) -> Self {
        Self { assigment }
    }

    pub fn wrapping_add<const N: usize>(&self, a: &[A; N], b: &[A; N]) -> [A; N] {
        let mut s = [A::ZERO; N];
        if N == 0 {
            return s;
        }
        let mut c = [A::ZERO; N];
        for i in 0..N - 1 {
            let acbc = (&a[i] + &c[i]) * (&b[i] + &c[i]);
            self.assigment.push(acbc.clone());
            (s[i], c[i + 1]) = (&a[i] + &b[i] + &c[i], &c[i] + acbc);
        }
        s[N - 1] = &a[N - 1] + &b[N - 1] + &c[N - 1];
        s
    }

    pub fn rotate_right<const N: usize>(&self, a: &[A; N], n: u32) -> [A; N] {
        let n = n as usize % N;
        let (a_l, a_r) = a.split_at(n);
        let mut o = [A::ZERO; N];
        let (o_l, o_r) = o.split_at_mut(N - n);
        o_l.clone_from_slice(a_r);
        o_r.clone_from_slice(a_l);
        o
    }

    pub fn bitxor<const N: usize>(&self, a: &[A; N], b: &[A; N]) -> [A; N] {
        array::from_fn(|i| &a[i] + &b[i])
    }
}
