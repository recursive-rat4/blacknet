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
use crate::assigner::assigment::Assigment;
use crate::assigner::polynomial::UnivariatePolynomial;
use crate::assigner::random::Distribution;
use crate::polynomial::{MultivariatePolynomial, Polynomial};
use crate::symmetric::{Absorb, Duplexer};
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct Proof<'a, R: UnitalRing> {
    claims: Vec<R>,
    assigment: &'a Assigment<R>,
}

impl<'a, R: UnitalRing> Proof<'a, R> {
    pub const fn new(claims: Vec<R>, assigment: &'a Assigment<R>) -> Self {
        Self { claims, assigment }
    }

    pub fn recover(&self, index: usize, degree: usize, sum: &R) -> UnivariatePolynomial<'a, R>
    where
        R: Clone,
        for<'b> &'b R: RingOps<R>,
    {
        let claim = &self.claims[index * degree..(index + 1) * degree];
        let mut coefficients = Vec::<R>::with_capacity(degree + 1);
        coefficients.push(claim[0].clone());
        coefficients.push(sum - (&claim[0]).double());
        for coefficient in claim.iter().skip(1) {
            coefficients[1] -= coefficient;
            coefficients.push(coefficient.clone());
        }
        UnivariatePolynomial::new(coefficients, self.assigment)
    }
}

pub struct SumCheck<
    'a,
    R: UnitalRing,
    P: MultivariatePolynomial<Coefficient = R, Point: From<Vec<R>>>,
    D: Duplexer,
    E: Distribution<'a, R, D, Output = R>,
> {
    _assigment: &'a Assigment<R>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<
    'a,
    R: UnitalRing + Absorb<D::Msg> + Clone,
    P: MultivariatePolynomial<Coefficient = R, Point: From<Vec<R>>>,
    D: Duplexer,
    E: Distribution<'a, R, D, Output = R>,
> SumCheck<'a, R, P, D, E>
{
    pub const fn new(assigment: &'a Assigment<R>) -> Self {
        Self {
            _assigment: assigment,
            phantom_p: PhantomData,
            phantom_d: PhantomData,
            phantom_e: PhantomData,
        }
    }

    pub fn verify_early_stopping(
        &self,
        polynomial: &P,
        mut sum: R,
        proof: &Proof<'a, R>,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> (P::Point, R)
    where
        for<'b> &'b R: RingOps<R>,
    {
        let mut coordinates = Vec::<R>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = proof.recover(i, polynomial.degree(), &sum);
            duplex.absorb(&claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.point(&challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = P::Point::from(coordinates);
        (r, sum)
    }
}
