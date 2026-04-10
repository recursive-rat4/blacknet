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

use crate::algebra::{AlgebraOps, UnitalAlgebra, UnitalRing};
use crate::polynomial::{
    MultivariatePolynomial, Polynomial, UnivariatePolynomial, interpolation::*,
};
use crate::random::Distribution;
use crate::symmetric::{Absorb, Duplexer};
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};

// https://users.cs.fiu.edu/~giri/teach/5420/f01/LundIPS.pdf

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Proof<R: UnitalRing> {
    claims: Vec<UnivariatePolynomial<R>>,
}

impl<R: UnitalRing> Proof<R> {
    pub const fn new(claims: Vec<UnivariatePolynomial<R>>) -> Self {
        Self { claims }
    }

    pub fn claim(&self, index: usize) -> &UnivariatePolynomial<R> {
        &self.claims[index]
    }

    pub const fn variables(&self) -> usize {
        self.claims.len()
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

pub struct SumCheck<
    R: UnitalRing,
    A: UnitalAlgebra<R>,
    P: MultivariatePolynomial<Coefficient = A, Point: From<Vec<A>>>,
    D: Duplexer,
    E: Distribution<D, Output = A>,
> {
    phantom_r: PhantomData<R>,
    phantom_a: PhantomData<A>,
    phantom_p: PhantomData<P>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<
    R: UnitalRing + InterpolationConsts,
    A: UnitalAlgebra<R> + Absorb<D::Msg> + Clone + Eq + Send + Sync,
    P: MultivariatePolynomial<Coefficient = A, Point: From<Vec<A>>> + Send + Sync,
    D: Duplexer,
    E: Distribution<D, Output = A>,
> SumCheck<R, A, P, D, E>
where
    for<'a> &'a A: AlgebraOps<R, A>,
{
    pub fn prove(
        mut polynomial: P,
        mut sum: A,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> Proof<A> {
        let mut claims = Vec::<UnivariatePolynomial<A>>::with_capacity(polynomial.variables());
        for _ in 0..polynomial.variables() {
            let claim = Self::prove_round(&polynomial, sum);
            duplex.absorb(&claim);
            let challenge = exceptional_set.sample(duplex);
            polynomial.bind(&challenge);
            sum = claim.point(&challenge);
            claims.push(claim);
            exceptional_set.reset();
        }
        Proof::new(claims)
    }

    pub fn verify(
        polynomial: &P,
        sum: A,
        proof: &Proof<A>,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> Result<(), Error<A>> {
        let (r, s) = Self::verify_early_stopping(polynomial, sum, proof, duplex, exceptional_set)?;
        let eval = polynomial.point(&r);
        if eval != s {
            return Err(Error::PolynomialIdentity(eval, s));
        }
        Ok(())
    }

    pub fn verify_early_stopping(
        polynomial: &P,
        mut sum: A,
        proof: &Proof<A>,
        duplex: &mut D,
        exceptional_set: &mut E,
    ) -> Result<(P::Point, A), Error<A>> {
        if proof.variables() != polynomial.variables() {
            return Err(Error::Variables(proof.variables(), polynomial.variables()));
        }
        let mut coordinates = Vec::<A>::with_capacity(polynomial.variables());
        for i in 0..polynomial.variables() {
            let claim = proof.claim(i);
            if claim.len() != polynomial.degree() + 1 {
                return Err(Error::Degree(i, claim.len(), polynomial.degree() + 1));
            }
            if claim.at_0_plus_1() != sum {
                return Err(Error::Sum(i, claim.at_0_plus_1(), sum));
            }
            duplex.absorb(claim);
            let challenge = exceptional_set.sample(duplex);
            sum = claim.point(&challenge);
            coordinates.push(challenge);
            exceptional_set.reset();
        }
        let r = P::Point::from(coordinates);
        Ok((r, sum))
    }

    fn prove_round(polynomial: &P, sum: A) -> UnivariatePolynomial<A> {
        if polynomial.degree() == 5 {
            let n2 = polynomial.sum_with_var::<-2>();
            let n1 = polynomial.sum_with_var::<-1>();
            let p1 = polynomial.sum_with_var::<1>();
            let p2 = polynomial.sum_with_var::<2>();
            let p3 = polynomial.sum_with_var::<3>();
            interpolate_5::<R, A>(n2, n1, sum - &p1, p1, p2, p3)
        } else if polynomial.degree() == 4 {
            let n2 = polynomial.sum_with_var::<-2>();
            let n1 = polynomial.sum_with_var::<-1>();
            let p1 = polynomial.sum_with_var::<1>();
            let p2 = polynomial.sum_with_var::<2>();
            interpolate_4::<R, A>(n2, n1, sum - &p1, p1, p2)
        } else if polynomial.degree() == 3 {
            let n1 = polynomial.sum_with_var::<-1>();
            let p1 = polynomial.sum_with_var::<1>();
            let p2 = polynomial.sum_with_var::<2>();
            interpolate_3::<R, A>(n1, sum - &p1, p1, p2)
        } else if polynomial.degree() == 2 {
            let n1 = polynomial.sum_with_var::<-1>();
            let p1 = polynomial.sum_with_var::<1>();
            interpolate_2::<R, A>(n1, sum - &p1, p1)
        } else if polynomial.degree() == 1 {
            let p1 = polynomial.sum_with_var::<1>();
            interpolate_1::<A>(sum - &p1, p1)
        } else {
            unimplemented!("Sum-check prover for degree {}", polynomial.degree());
        }
    }
}

#[derive(Debug)]
pub enum Error<R: UnitalRing> {
    Variables(usize, usize),
    Degree(usize, usize, usize),
    Sum(usize, R, R),
    PolynomialIdentity(R, R),
}

impl<R: UnitalRing> fmt::Display for Error<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Variables(actual, expected) => {
                write!(f, "Expected {expected} claims got {actual}")
            }
            Error::Degree(round, actual, expected) => write!(
                f,
                "At round {round} expected {expected} degree claim got {actual}"
            ),
            Error::Sum(round, _, _) => write!(f, "Partial sum at round {round} doesn't match"),
            Error::PolynomialIdentity(_, _) => write!(f, "Polynomial identity check failed"),
        }
    }
}

impl<R: UnitalRing + fmt::Debug> core::error::Error for Error<R> {}
