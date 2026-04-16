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

use crate::algebra::AdditiveGroup;
use crate::assigner::assigment::Assigment;
use crate::assigner::symmetric::Permutation;
use crate::random::UniformGenerator;
use crate::symmetric::{Duplexer, Phase};
use core::marker::PhantomData;
use zeroize::Zeroize;

#[derive(Zeroize)]
pub struct Duplex<
    'a,
    S: AdditiveGroup + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> {
    phase: Phase,
    position: usize,
    state: [S; WIDTH],
    phantom: PhantomData<P>,
    #[zeroize(skip)]
    assigment: &'a Assigment<S>,
}

impl<
    'a,
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> Duplex<'a, S, RATE, CAPACITY, WIDTH, P>
{
    pub const fn new(assigment: &'a Assigment<S>) -> Self {
        Self {
            phase: Phase::Absorb,
            position: 0,
            state: [S::ZERO; WIDTH],
            phantom: PhantomData,
            assigment,
        }
    }

    pub fn with_iv(assigment: &'a Assigment<S>, iv: &[S; CAPACITY]) -> Self {
        let mut duplex = Self::new(assigment);
        duplex.state[RATE..WIDTH].clone_from_slice(iv);
        duplex
    }

    pub fn reset_with_iv(&mut self, iv: &[S; CAPACITY]) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state[..RATE].fill(S::ZERO);
        self.state[RATE..WIDTH].clone_from_slice(iv);
    }

    fn pad(&mut self) {
        if self.position != RATE {
            self.state[self.position] = S::from(1);
            self.position += 1;
            self.state[self.position..RATE].fill(S::ZERO);
            self.position = RATE;
            self.state[WIDTH - 1] += S::from(2);
        } else {
            self.state[WIDTH - 1] += S::from(1);
        }
    }
}

impl<
    'a,
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> Clone for Duplex<'a, S, RATE, CAPACITY, WIDTH, P>
{
    fn clone(&self) -> Self {
        Self {
            phase: self.phase,
            position: self.position,
            state: self.state.clone(),
            phantom: PhantomData,
            assigment: self.assigment,
        }
    }
}

impl<
    'a,
    S: AdditiveGroup + Copy + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> Copy for Duplex<'a, S, RATE, CAPACITY, WIDTH, P>
{
}

impl<
    'a,
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> Duplexer for Duplex<'a, S, RATE, CAPACITY, WIDTH, P>
{
    type Msg = S;

    fn reset(&mut self) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state = [S::ZERO; WIDTH];
    }

    fn absorb_msg(&mut self, e: S) {
        if self.phase == Phase::Squeeze {
            self.phase = Phase::Absorb;
            self.position = 0;
        } else if self.position == RATE {
            P::permute(self.assigment, &mut self.state);
            self.position = 0;
        }
        self.state[self.position] = e;
        self.position += 1
    }

    fn squeeze_msg(&mut self) -> S {
        if self.phase == Phase::Absorb {
            self.phase = Phase::Squeeze;
            self.pad();
            P::permute(self.assigment, &mut self.state);
            self.position = 0;
        } else if self.position == RATE {
            P::permute(self.assigment, &mut self.state);
            self.position = 0;
        }
        let e = self.state[self.position].clone();
        self.position += 1;
        e
    }
}

impl<
    'a,
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> UniformGenerator for Duplex<'a, S, RATE, CAPACITY, WIDTH, P>
{
    type Output = S;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.squeeze_msg()
    }
}
