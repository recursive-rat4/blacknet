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

use crate::algebra::{AlgebraOps, UnitalAlgebra};
use crate::circuit::builder::{CircuitBuilder, LinearCombination};
use crate::gf2::GF2;
use core::array;

/// Arithmetic gates over GF(2)-algebra.
pub struct ArithmeticGate<'a, 'b, A: UnitalAlgebra<GF2>> {
    circuit: &'a CircuitBuilder<'b, A>,
}

impl<'a, 'b, A: UnitalAlgebra<GF2> + Clone + Eq> ArithmeticGate<'a, 'b, A>
where
    for<'c> &'c A: AlgebraOps<GF2, A>,
{
    pub const fn new(circuit: &'a CircuitBuilder<'b, A>) -> Self {
        Self { circuit }
    }

    pub fn wrapping_add<const N: usize>(
        &self,
        a: &[LinearCombination<A>; N],
        b: &[LinearCombination<A>; N],
    ) -> [LinearCombination<A>; N] {
        let mut s = [LinearCombination::ZERO; N];
        if N == 0 {
            return s;
        }
        let scope = self.circuit.scope("ArithmeticGate::wrapping_add");
        let mut c = [LinearCombination::ZERO; N];
        for i in 0..N - 1 {
            let acbc = scope.auxiliary();
            scope.constrain((&a[i] + &c[i]) * (&b[i] + &c[i]), acbc);
            (s[i], c[i + 1]) = (&a[i] + &b[i] + &c[i], &c[i] + acbc);
        }
        s[N - 1] = &a[N - 1] + &b[N - 1] + &c[N - 1];
        s
    }

    pub fn rotate_right<const N: usize>(
        &self,
        a: &[LinearCombination<A>; N],
        n: u32,
    ) -> [LinearCombination<A>; N] {
        let n = n as usize % N;
        let (a_l, a_r) = a.split_at(n);
        let mut o = [LinearCombination::ZERO; N];
        let (o_l, o_r) = o.split_at_mut(N - n);
        o_l.clone_from_slice(a_r);
        o_r.clone_from_slice(a_l);
        o
    }

    pub fn bitxor<const N: usize>(
        &self,
        a: &[LinearCombination<A>; N],
        b: &[LinearCombination<A>; N],
    ) -> [LinearCombination<A>; N] {
        array::from_fn(|i| &a[i] + &b[i])
    }
}
