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

use crate::circuit::builder::{CircuitBuilder, LinearCombination, Scope};
use crate::gf2::GF2;
use core::array;

/// Arithmetic gates over `GF(2)`.
pub struct ArithmeticGate<'a, 'b> {
    circuit: &'a CircuitBuilder<'b, GF2>,
}

impl<'a, 'b> ArithmeticGate<'a, 'b> {
    pub const fn new(circuit: &'a CircuitBuilder<'b, GF2>) -> Self {
        Self { circuit }
    }

    fn half_adder(
        scope: &Scope<GF2>,
        a: &LinearCombination<GF2>,
        b: &LinearCombination<GF2>,
    ) -> (LinearCombination<GF2>, LinearCombination<GF2>) {
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);

        let s = a + b;
        let c = LinearCombination::from(ab);
        (s, c)
    }

    fn full_adder(
        scope: &Scope<GF2>,
        a: &LinearCombination<GF2>,
        b: &LinearCombination<GF2>,
        c: &LinearCombination<GF2>,
    ) -> (LinearCombination<GF2>, LinearCombination<GF2>) {
        let ab = scope.auxiliary();
        scope.constrain(a * b, ab);
        let cab = scope.auxiliary();
        scope.constrain(c * (a + b), cab);

        let s = a + b + c;
        let c = ab + cab;
        (s, c)
    }

    pub fn wrapping_add<const N: usize>(
        &self,
        a: &[LinearCombination<GF2>; N],
        b: &[LinearCombination<GF2>; N],
    ) -> [LinearCombination<GF2>; N] {
        let mut s = [const { LinearCombination::new() }; N];
        if N == 0 {
            return s;
        }
        // Ripple-carry adder
        let scope = self.circuit.scope("ArithmeticGate::wrapping_add");
        let mut c: LinearCombination<GF2>;
        (s[0], c) = Self::half_adder(&scope, &a[0], &b[0]);
        for i in 1..N {
            (s[i], c) = Self::full_adder(&scope, &a[i], &b[i], &c);
        }
        s
    }

    pub fn rotate_right<const N: usize>(
        &self,
        a: &[LinearCombination<GF2>; N],
        n: u32,
    ) -> [LinearCombination<GF2>; N] {
        let n = n as usize % N;
        let (a_l, a_r) = a.split_at(n);
        let mut o = [const { LinearCombination::new() }; N];
        let (o_l, o_r) = o.split_at_mut(N - n);
        o_l.clone_from_slice(a_r);
        o_r.clone_from_slice(a_l);
        o
    }

    pub fn bitxor<const N: usize>(
        &self,
        a: &[LinearCombination<GF2>; N],
        b: &[LinearCombination<GF2>; N],
    ) -> [LinearCombination<GF2>; N] {
        array::from_fn(|i| &a[i] + &b[i])
    }
}
