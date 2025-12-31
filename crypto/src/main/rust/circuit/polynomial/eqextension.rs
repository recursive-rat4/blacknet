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

use crate::algebra::Double;
use crate::algebra::UnitalRing;
use crate::circuit::builder::{CircuitBuilder, Constant, LinearCombination, VariableKind};
use crate::circuit::polynomial::Point;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::zip;

pub struct EqExtension<'a, 'b, R: UnitalRing> {
    circuit: &'a CircuitBuilder<'b, R>,
    coefficients: Vec<LinearCombination<R>>,
}

impl<'a, 'b, R: UnitalRing + Eq> EqExtension<'a, 'b, R> {
    pub const fn new(
        circuit: &'a CircuitBuilder<'b, R>,
        coefficients: Vec<LinearCombination<R>>,
    ) -> Self {
        Self {
            circuit,
            coefficients,
        }
    }

    pub fn allocate(
        circuit: &'a CircuitBuilder<'b, R>,
        kind: VariableKind,
        variables: usize,
    ) -> Self {
        let scope = circuit.scope("EqExtension::allocate");
        Self {
            circuit,
            coefficients: (0..variables)
                .map(|_| scope.variable(kind).into())
                .collect(),
        }
    }

    pub fn point(&self, point: &Point<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("EqExtension::point");
        let mut pi = LinearCombination::<R>::from(Constant::ONE);
        zip(self.coefficients.iter(), point.coordinates()).for_each(|(c, p)| {
            let cp = scope.auxiliary();
            scope.constrain(c * p, cp);
            let t = scope.auxiliary();
            scope.constrain(&pi * (cp.double() - c - p + Constant::ONE), t);
            pi = t.into();
        });
        pi
    }

    pub fn hypercube(&self) -> Vec<LinearCombination<R>> {
        let scope = self.circuit.scope("EqExtension::hypercube");
        let mut r = vec![LinearCombination::new(); 1 << self.coefficients.len()];
        r[0] = Constant::ONE.into();
        let mut j = 1;
        for i in (0..self.coefficients.len()).rev() {
            let mut l = j;
            for k in 0..j {
                let t = scope.auxiliary();
                scope.constrain(&self.coefficients[i] * &r[k], t);
                r[l] = t.into();
                r[k] -= t;
                l += 1;
            }
            j <<= 1;
        }
        r
    }
}
