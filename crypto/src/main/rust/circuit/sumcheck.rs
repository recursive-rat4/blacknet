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

use crate::circuit::builder::{CircuitBuilder, LinearCombination, VariableKind};
use crate::circuit::distribution::Distribution;
use crate::circuit::polynomial::{Point, UnivariatePolynomial};
use crate::duplex::Duplex;
use crate::polynomial::Polynomial;
use crate::ring::UnitalRing;
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct Proof<'a, 'b, R: UnitalRing> {
    claims: Vec<UnivariatePolynomial<'a, 'b, R>>,
}

impl<'a, 'b, R: UnitalRing + Eq> Proof<'a, 'b, R> {
    pub fn allocate(
        circuit: &'a CircuitBuilder<'b, R>,
        kind: VariableKind,
        variables: usize,
        degree: usize,
    ) -> Self {
        Self {
            claims: (0..variables)
                .map(|_| UnivariatePolynomial::allocate(circuit, kind, degree))
                .collect(),
        }
    }
}

impl<'a, 'b, R: UnitalRing> From<Vec<UnivariatePolynomial<'a, 'b, R>>> for Proof<'a, 'b, R> {
    fn from(claims: Vec<UnivariatePolynomial<'a, 'b, R>>) -> Self {
        Self { claims }
    }
}

pub struct SumCheck<
    'a,
    'b,
    R: UnitalRing,
    P: Polynomial<R>,
    D: Duplex<LinearCombination<R>>,
    E: Distribution<'a, 'b, R, D, Output = LinearCombination<R>>,
> {
    circuit: &'a CircuitBuilder<'b, R>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<
    'a,
    'b,
    R: UnitalRing + Eq,
    P: Polynomial<R>,
    D: Duplex<LinearCombination<R>>,
    E: Distribution<'a, 'b, R, D, Output = LinearCombination<R>>,
> SumCheck<'a, 'b, R, P, D, E>
{
    pub const fn new(circuit: &'a CircuitBuilder<'b, R>) -> Self {
        Self {
            circuit,
            phantom_p: PhantomData,
            phantom_d: PhantomData,
            phantom_e: PhantomData,
        }
    }

    pub fn verify_early_stopping(
        &self,
        polynomial: &P,
        mut sum: LinearCombination<R>,
        proof: &Proof<'a, 'b, R>,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> (Point<R>, LinearCombination<R>) {
        let scope = self.circuit.scope("SumCheck::verify_early_stopping");
        let mut coordinates = Vec::<LinearCombination<R>>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = &proof.claims[i];
            scope.constrain(claim.at_0_plus_1(), sum.clone());
            duplex.absorb(claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.evaluate(&challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = Point::from(coordinates);
        (r, sum)
    }
}
