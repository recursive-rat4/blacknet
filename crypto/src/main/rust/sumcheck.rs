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

use crate::algebra::UnitalRing;
use crate::distribution::Distribution;
use crate::duplex::Duplex;
use crate::polynomial::{Point, Polynomial, UnivariatePolynomial, interpolation::*};
use alloc::vec::Vec;
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// https://users.cs.fiu.edu/~giri/teach/5420/f01/LundIPS.pdf

#[derive(Deserialize, Serialize)]
pub struct Proof<R: UnitalRing> {
    claims: Vec<UnivariatePolynomial<R>>,
}

impl<R: UnitalRing> Proof<R> {
    pub fn claim(&self, index: usize) -> &UnivariatePolynomial<R> {
        &self.claims[index]
    }

    pub const fn variables(&self) -> usize {
        self.claims.len()
    }
}

impl<R: UnitalRing> From<Vec<UnivariatePolynomial<R>>> for Proof<R> {
    fn from(claims: Vec<UnivariatePolynomial<R>>) -> Self {
        Self { claims }
    }
}

impl<'a, R: UnitalRing> IntoIterator for &'a Proof<R> {
    type Item = &'a UnivariatePolynomial<R>;
    type IntoIter = core::slice::Iter<'a, UnivariatePolynomial<R>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.claims.iter()
    }
}

pub struct SumCheck<R: UnitalRing, P: Polynomial<R>, D: Duplex<R>, E: Distribution<D, Output = R>> {
    phantom_r: PhantomData<R>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<
    R: UnitalRing + InterpolationConsts + Eq + Send + Sync,
    P: Polynomial<R> + Send + Sync,
    D: Duplex<R>,
    E: Distribution<D, Output = R>,
> SumCheck<R, P, D, E>
{
    pub fn prove(
        mut polynomial: P,
        mut sum: R,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> Proof<R> {
        let mut claims = Vec::<UnivariatePolynomial<R>>::with_capacity(polynomial.variables());
        for _ in 0..polynomial.variables() {
            let claim = Self::prove_round(&polynomial, sum);
            duplex.absorb(&claim);
            let challenge = exceptional_set.sample(duplex);
            polynomial.bind(challenge);
            sum = claim.evaluate(challenge);
            claims.push(claim);
            exceptional_set.reset();
        }
        claims.into()
    }

    pub fn verify(
        polynomial: &P,
        mut sum: R,
        proof: &Proof<R>,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> Result<(), Error<R>> {
        if proof.variables() != polynomial.variables() {
            return Err(Error::Variables(proof.variables(), polynomial.variables()));
        }
        let mut coordinates = Vec::<R>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = proof.claim(i);
            if claim.degree() != polynomial.degree() {
                return Err(Error::Degree(i, claim.degree(), polynomial.degree()));
            }
            if claim.at_0_plus_1() != sum {
                return Err(Error::Sum(i, claim.at_0_plus_1(), sum));
            }
            duplex.absorb(claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.evaluate(challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = Point::new(coordinates);
        if polynomial.point(&r) != sum {
            return Err(Error::PolynomialIdentity(polynomial.point(&r), sum));
        }
        Ok(())
    }

    pub fn verify_early_stopping(
        polynomial: &P,
        mut sum: R,
        proof: &Proof<R>,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> Result<(Point<R>, R), Error<R>> {
        if proof.variables() != polynomial.variables() {
            return Err(Error::Variables(proof.variables(), polynomial.variables()));
        }
        let mut coordinates = Vec::<R>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = proof.claim(i);
            if claim.degree() != polynomial.degree() {
                return Err(Error::Degree(i, claim.degree(), polynomial.degree()));
            }
            if claim.at_0_plus_1() != sum {
                return Err(Error::Sum(i, claim.at_0_plus_1(), sum));
            }
            duplex.absorb(claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.evaluate(challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = Point::new(coordinates);
        Ok((r, sum))
    }

    fn prove_round(polynomial: &P, sum: R) -> UnivariatePolynomial<R> {
        if polynomial.degree() == 5 {
            let evaluations = polynomial.hypercube_with_var::<-2>();
            let n2 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<2>();
            let p2 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<3>();
            let p3 = evaluations.into_iter().sum::<R>();
            interpolate_5(n2, n1, sum - p1, p1, p2, p3)
        } else if polynomial.degree() == 4 {
            let evaluations = polynomial.hypercube_with_var::<-2>();
            let n2 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<2>();
            let p2 = evaluations.into_iter().sum::<R>();
            interpolate_4(n2, n1, sum - p1, p1, p2)
        } else if polynomial.degree() == 3 {
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<2>();
            let p2 = evaluations.into_iter().sum::<R>();
            interpolate_3(n1, sum - p1, p1, p2)
        } else if polynomial.degree() == 2 {
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.into_iter().sum::<R>();
            interpolate_2(n1, sum - p1, p1)
        } else if polynomial.degree() == 1 {
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.into_iter().sum::<R>();
            interpolate_1(sum - p1, p1)
        } else {
            unimplemented!("Sum-check prover for degree {}", polynomial.degree());
        }
    }
}

#[derive(Debug, Error)]
pub enum Error<R: UnitalRing> {
    #[error("Expected {1} claims got {0}")]
    Variables(usize, usize),
    #[error("At round {0} expected {2} degree claim got {1}")]
    Degree(usize, usize, usize),
    #[error("Partial sum at round {0} doesn't match")]
    Sum(usize, R, R),
    #[error("Polynomial identity check failed")]
    PolynomialIdentity(R, R),
}
