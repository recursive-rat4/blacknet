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

use crate::circuit::circuitbuilder::{CircuitBuilder, LinearCombination, VariableKind};
use crate::duplex::{Absorb, Duplex};
use crate::ring::UnitalRing;
use core::ops::Add;

pub struct UnivariatePolynomial<'a, R: UnitalRing> {
    circuit: &'a CircuitBuilder<R>,
    coefficients: Vec<LinearCombination<R>>,
}

impl<'a, R: UnitalRing> UnivariatePolynomial<'a, R> {
    pub fn allocate(circuit: &'a CircuitBuilder<R>, kind: VariableKind, degree: usize) -> Self {
        let scope = circuit.scope("UnivariatePolynomial::allocate");
        Self {
            circuit,
            coefficients: (0..degree + 1)
                .map(|_| scope.variable(kind).into())
                .collect(),
        }
    }

    pub fn evaluate(&self, point: &LinearCombination<R>) -> LinearCombination<R> {
        let scope = self.circuit.scope("UnivariatePolynomial::evaluate");
        let mut sigma = self.coefficients[0].clone();
        let mut power = point.clone();
        for i in 1..self.coefficients.len() - 1 {
            let cp = scope.auxiliary();
            scope.constrain(&self.coefficients[i] * &power, cp);
            sigma += cp;
            let pp = scope.auxiliary();
            scope.constrain(power * point, pp);
            power = pp.into();
        }
        if self.coefficients.len() > 1 {
            let cp = scope.auxiliary();
            scope.constrain(&self.coefficients[self.coefficients.len() - 1] * power, cp);
            sigma += cp;
        }
        sigma
    }

    pub fn at_0_plus_1(&self) -> LinearCombination<R> {
        self.coefficients
            .iter()
            .fold(self.coefficients[0].clone(), Add::add)
    }
}

impl<'a, R: UnitalRing> Absorb<LinearCombination<R>> for UnivariatePolynomial<'a, R> {
    fn absorb_into(&self, duplex: &mut (impl Duplex<LinearCombination<R>> + ?Sized)) {
        duplex.absorb(&self.coefficients)
    }
}
