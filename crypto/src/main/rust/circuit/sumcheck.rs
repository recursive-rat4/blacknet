/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use crate::algebra::{Double, RingOps, UnitalRing};
use crate::circuit::builder::{CircuitBuilder, LinearCombination, VariableKind};
use crate::circuit::polynomial::{Point, UnivariatePolynomial};
use crate::circuit::random::Distribution;
use crate::polynomial::{MultivariatePolynomial, Polynomial};
use crate::symmetric::Duplexer;
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct Proof<'a, 'b, R: UnitalRing> {
    circuit: &'a CircuitBuilder<'b, R>,
    claims: Vec<LinearCombination<R>>,
}

impl<'a, 'b, R: UnitalRing + Clone + Eq> Proof<'a, 'b, R> {
    pub fn allocate(
        circuit: &'a CircuitBuilder<'b, R>,
        kind: VariableKind,
        variables: usize,
        degree: usize,
    ) -> Self {
        let scope = circuit.scope("Proof::allocate");
        Self {
            circuit,
            claims: (0..degree * variables)
                .map(|_| scope.variable(kind).into())
                .collect(),
        }
    }

    pub const fn new(
        circuit: &'a CircuitBuilder<'b, R>,
        claims: Vec<LinearCombination<R>>,
    ) -> Self {
        Self { circuit, claims }
    }

    pub fn recover(
        &self,
        index: usize,
        degree: usize,
        sum: &LinearCombination<R>,
    ) -> UnivariatePolynomial<'a, 'b, R>
    where
        for<'c> &'c R: RingOps<R>,
    {
        let claim = &self.claims[index * degree..(index + 1) * degree];
        let mut coefficients = Vec::<LinearCombination<R>>::with_capacity(degree + 1);
        coefficients.push(claim[0].clone());
        coefficients.push(sum - (&claim[0]).double());
        for coefficient in claim.iter().skip(1) {
            coefficients[1] -= coefficient;
            coefficients.push(coefficient.clone());
        }
        UnivariatePolynomial::new(self.circuit, coefficients)
    }
}

pub struct SumCheck<
    'a,
    'b,
    R: UnitalRing,
    P: MultivariatePolynomial<Coefficient = R>,
    D: Duplexer<Msg = LinearCombination<R>>,
    E: Distribution<'a, 'b, R, D, Output = LinearCombination<R>>,
> {
    _circuit: &'a CircuitBuilder<'b, R>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<
    'a,
    'b,
    R: UnitalRing + Clone + Eq,
    P: MultivariatePolynomial<Coefficient = R>,
    D: Duplexer<Msg = LinearCombination<R>>,
    E: Distribution<'a, 'b, R, D, Output = LinearCombination<R>>,
> SumCheck<'a, 'b, R, P, D, E>
where
    for<'c> &'c R: RingOps<R>,
{
    pub const fn new(circuit: &'a CircuitBuilder<'b, R>) -> Self {
        Self {
            _circuit: circuit,
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
        let mut coordinates = Vec::<LinearCombination<R>>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = proof.recover(i, polynomial.degree(), &sum);
            duplex.absorb(&claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.point(&challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = Point::new(coordinates);
        (r, sum)
    }
}
