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

use crate::assigner::assigment::Assigment;
use crate::assigner::distribution::Distribution;
use crate::assigner::univariatepolynomial::UnivariatePolynomial;
use crate::duplex::Duplex;
use crate::point::Point;
use crate::polynomial::Polynomial;
use crate::ring::UnitalRing;
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct Proof<'a, R: UnitalRing> {
    claims: Vec<UnivariatePolynomial<'a, R>>,
}

impl<'a, R: UnitalRing> From<Vec<UnivariatePolynomial<'a, R>>> for Proof<'a, R> {
    fn from(claims: Vec<UnivariatePolynomial<'a, R>>) -> Self {
        Self { claims }
    }
}

pub struct SumCheck<
    'a,
    R: UnitalRing,
    P: Polynomial<R>,
    D: Duplex<R>,
    E: Distribution<'a, R, D, Output = R>,
> {
    _assigment: &'a Assigment<R>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<'a, R: UnitalRing, P: Polynomial<R>, D: Duplex<R>, E: Distribution<'a, R, D, Output = R>>
    SumCheck<'a, R, P, D, E>
{
    pub fn new(assigment: &'a Assigment<R>) -> Self {
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
    ) -> (Point<R>, R) {
        let mut coordinates = Vec::<R>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = &proof.claims[i];
            duplex.absorb(claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.evaluate(challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = Point::from(coordinates);
        (r, sum)
    }
}
