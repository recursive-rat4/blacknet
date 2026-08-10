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
use crate::circuit::builder::{CircuitBuilder, LinearCombination, VariableKind};
use crate::gf2::GF2;
use core::array;

/// Unsigned int over GF(2)-algebra.
pub struct UInt<'a, A: UnitalAlgebra<GF2>, const N: usize> {
    circuit: &'a CircuitBuilder<A>,
    bits: [LinearCombination<A>; N],
}

impl<'a, A: UnitalAlgebra<GF2> + Clone + Eq, const N: usize> UInt<'a, A, N>
where
    for<'b> &'b A: AlgebraOps<GF2, A>,
{
    pub fn allocate(circuit: &'a CircuitBuilder<A>, kind: VariableKind) -> Self {
        let scope = circuit.scope("UInt::allocate");
        Self {
            circuit,
            bits: array::from_fn(|_| scope.variable(kind).into()),
        }
    }

    pub const fn new(circuit: &'a CircuitBuilder<A>, bits: [LinearCombination<A>; N]) -> Self {
        Self { circuit, bits }
    }

    pub const fn zero(circuit: &'a CircuitBuilder<A>) -> Self {
        Self {
            circuit,
            bits: [LinearCombination::ZERO; N],
        }
    }

    pub fn wrapping_add(&self, rps: &Self) -> Self {
        let mut bits = [LinearCombination::ZERO; N];
        if N == 0 {
            return Self {
                circuit: self.circuit,
                bits,
            };
        }
        let scope = self.circuit.scope("UInt::wrapping_add");
        let mut c = [LinearCombination::ZERO; N];
        for i in 0..N - 1 {
            let acbc = scope.auxiliary();
            scope.constrain((&self.bits[i] + &c[i]) * (&rps.bits[i] + &c[i]), acbc);
            (bits[i], c[i + 1]) = (&self.bits[i] + &rps.bits[i] + &c[i], &c[i] + acbc);
        }
        bits[N - 1] = &self.bits[N - 1] + &rps.bits[N - 1] + &c[N - 1];
        Self {
            circuit: self.circuit,
            bits,
        }
    }

    pub fn rotate_right(&self, n: u32) -> Self {
        let n = n as usize % N;
        let (a_l, a_r) = self.bits.split_at(n);
        let mut bits = [LinearCombination::ZERO; N];
        let (o_l, o_r) = bits.split_at_mut(N - n);
        o_l.clone_from_slice(a_r);
        o_r.clone_from_slice(a_l);
        Self {
            circuit: self.circuit,
            bits,
        }
    }

    pub fn bitxor(&self, rps: &Self) -> Self {
        Self {
            circuit: self.circuit,
            bits: array::from_fn(|i| &self.bits[i] + &rps.bits[i]),
        }
    }
}

impl<A: UnitalAlgebra<GF2>, const N: usize> IntoIterator for UInt<'_, A, N> {
    type Item = LinearCombination<A>;
    type IntoIter = core::array::IntoIter<LinearCombination<A>, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bits.into_iter()
    }
}

impl<'a, A: UnitalAlgebra<GF2>, const N: usize> IntoIterator for &'a UInt<'_, A, N> {
    type Item = &'a LinearCombination<A>;
    type IntoIter = core::slice::Iter<'a, LinearCombination<A>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bits.iter()
    }
}

pub type UInt8<'a, A> = UInt<'a, A, 8>;
pub type UInt16<'a, A> = UInt<'a, A, 16>;
pub type UInt32<'a, A> = UInt<'a, A, 32>;
pub type UInt64<'a, A> = UInt<'a, A, 64>;
