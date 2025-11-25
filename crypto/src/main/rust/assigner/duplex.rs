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
use crate::assigner::permutation::Permutation;
use crate::distribution::UniformGenerator;
use crate::duplex::{Duplex, Phase};
use crate::semiring::Semiring;
use core::marker::PhantomData;

pub struct DuplexImpl<
    'a,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> {
    phase: Phase,
    position: usize,
    state: [S; WIDTH],
    phantom: PhantomData<P>,
    assigment: &'a Assigment<S>,
}

impl<
    'a,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> DuplexImpl<'a, S, RATE, CAPACITY, WIDTH, P>
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
        duplex.state[RATE..WIDTH].copy_from_slice(iv);
        duplex
    }

    pub fn reset_with_iv(&mut self, iv: &[S; CAPACITY]) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state[..RATE].fill(S::ZERO);
        self.state[RATE..WIDTH].copy_from_slice(iv);
    }

    fn pad(&mut self) {
        if self.position != RATE {
            self.state[self.position] = S::ONE;
            self.position += 1;
            self.state[self.position..RATE].fill(S::ZERO);
            self.position = RATE;
            self.state[WIDTH - 1] += S::from(2);
        } else {
            self.state[WIDTH - 1] += S::ONE;
        }
    }
}

impl<
    'a,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> Duplex<S> for DuplexImpl<'a, S, RATE, CAPACITY, WIDTH, P>
{
    fn reset(&mut self) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state = [S::ZERO; WIDTH];
    }

    fn absorb_native(&mut self, e: &S) {
        if self.phase == Phase::Squeeze {
            self.phase = Phase::Absorb;
            self.position = 0;
        } else if self.position == RATE {
            P::permute(self.assigment, &mut self.state);
            self.position = 0;
        }
        self.state[self.position] = *e;
        self.position += 1
    }

    fn squeeze_native(&mut self) -> S {
        if self.phase == Phase::Absorb {
            self.phase = Phase::Squeeze;
            self.pad();
            P::permute(self.assigment, &mut self.state);
            self.position = 0;
        } else if self.position == RATE {
            P::permute(self.assigment, &mut self.state);
            self.position = 0;
        }
        let e = self.state[self.position];
        self.position += 1;
        e
    }
}

impl<
    'a,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [S; WIDTH]>,
> UniformGenerator for DuplexImpl<'a, S, RATE, CAPACITY, WIDTH, P>
{
    type Output = S;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.squeeze_native()
    }
}
