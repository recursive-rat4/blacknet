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

use crate::distribution::Distribution;
use crate::duplex::Duplex;
use crate::interpolation::*;
use crate::point::Point;
use crate::polynomial::Polynomial;
use crate::ring::UnitalRing;
use crate::univariatepolynomial::UnivariatePolynomial;
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;

// https://users.cs.fiu.edu/~giri/teach/5420/f01/LundIPS.pdf

pub struct Proof<R: UnitalRing> {
    claims: Vec<UnivariatePolynomial<R>>,
}

impl<R: UnitalRing> Proof<R> {
    pub const fn claims(&self) -> &Vec<UnivariatePolynomial<R>> {
        &self.claims
    }
}

impl<R: UnitalRing> From<Vec<UnivariatePolynomial<R>>> for Proof<R> {
    fn from(claims: Vec<UnivariatePolynomial<R>>) -> Self {
        Self { claims }
    }
}

pub struct SumCheck<R: UnitalRing, P: Polynomial<R>, D: Duplex<R>, E: Distribution<D, Output = R>> {
    phantom_r: PhantomData<R>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<
    R: UnitalRing + InterpolationConsts,
    P: Polynomial<R>,
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
        if proof.claims.len() != polynomial.variables() {
            return Err(Error::Claims(proof.claims.len(), polynomial.variables()));
        }
        let mut coordinates = Vec::<R>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = &proof.claims[i];
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
        let r = Point::from(coordinates);
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
        if proof.claims.len() != polynomial.variables() {
            return Err(Error::Claims(proof.claims.len(), polynomial.variables()));
        }
        let mut coordinates = Vec::<R>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = &proof.claims[i];
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
        let r = Point::from(coordinates);
        Ok((r, sum))
    }

    fn prove_round(polynomial: &P, sum: R) -> UnivariatePolynomial<R> {
        if polynomial.degree() == 5 {
            let evaluations = polynomial.hypercube_with_var::<-2>();
            let n2 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<2>();
            let p2 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<3>();
            let p3 = evaluations.steal().into_iter().sum::<R>();
            interpolate_5(n2, n1, sum - p1, p1, p2, p3)
        } else if polynomial.degree() == 4 {
            let evaluations = polynomial.hypercube_with_var::<-2>();
            let n2 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<2>();
            let p2 = evaluations.steal().into_iter().sum::<R>();
            interpolate_4(n2, n1, sum - p1, p1, p2)
        } else if polynomial.degree() == 3 {
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<2>();
            let p2 = evaluations.steal().into_iter().sum::<R>();
            interpolate_3(n1, sum - p1, p1, p2)
        } else if polynomial.degree() == 2 {
            let evaluations = polynomial.hypercube_with_var::<-1>();
            let n1 = evaluations.steal().into_iter().sum::<R>();
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.steal().into_iter().sum::<R>();
            interpolate_2(n1, sum - p1, p1)
        } else if polynomial.degree() == 1 {
            let evaluations = polynomial.hypercube_with_var::<1>();
            let p1 = evaluations.steal().into_iter().sum::<R>();
            interpolate_1(sum - p1, p1)
        } else {
            unimplemented!("Sum-check prover for degree {}", polynomial.degree());
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error<R: UnitalRing> {
    Claims(usize, usize),
    Degree(usize, usize, usize),
    Sum(usize, R, R),
    PolynomialIdentity(R, R),
}

impl<R: UnitalRing> core::error::Error for Error<R> {}

impl<R: UnitalRing> fmt::Display for Error<R> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Claims(actual, expected) => {
                write!(formatter, "Expected {expected} claims got {actual}")
            }
            Error::Degree(round, actual, expected) => {
                write!(
                    formatter,
                    "At round {round} expected {expected} degree claim got {actual}"
                )
            }
            Error::Sum(round, _, _) => {
                write!(formatter, "Partial sum at round {round} doesn't match")
            }
            Error::PolynomialIdentity(_, _) => {
                write!(formatter, "Polynomial identity check failed")
            }
        }
    }
}
