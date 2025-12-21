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

use crate::circuit::builder::{CircuitBuilder, Constant, LinearCombination};
use crate::circuit::permutation::Permutation;
use crate::distribution::UniformGenerator;
use crate::duplex::{Duplex, Phase};
use crate::semiring::Semiring;
use core::array;
use core::marker::PhantomData;

pub struct DuplexImpl<
    'a,
    'b,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [LinearCombination<S>; WIDTH]>,
> {
    circuit: &'a CircuitBuilder<'b, S>,
    phase: Phase,
    position: usize,
    state: [LinearCombination<S>; WIDTH],
    phantom: PhantomData<P>,
}

impl<
    'a,
    'b,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [LinearCombination<S>; WIDTH]>,
> DuplexImpl<'a, 'b, S, RATE, CAPACITY, WIDTH, P>
{
    pub fn new(circuit: &'a CircuitBuilder<'b, S>) -> Self {
        Self {
            circuit,
            phase: Phase::Absorb,
            position: 0,
            state: array::from_fn(|_| LinearCombination::new()),
            phantom: PhantomData,
        }
    }

    pub fn with_iv(
        circuit: &'a CircuitBuilder<'b, S>,
        iv: &[LinearCombination<S>; CAPACITY],
    ) -> Self {
        let mut duplex = Self::new(circuit);
        duplex.state[RATE..WIDTH].clone_from_slice(iv);
        duplex
    }

    pub fn reset_with_iv(&mut self, iv: &[LinearCombination<S>; CAPACITY]) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state[..RATE]
            .iter_mut()
            .for_each(LinearCombination::clear);
        self.state[RATE..WIDTH].clone_from_slice(iv);
    }

    fn pad(&mut self) {
        if self.position != RATE {
            self.state[self.position] = Constant::ONE.into();
            self.position += 1;
            self.state[self.position..RATE]
                .iter_mut()
                .for_each(LinearCombination::clear);
            self.position = RATE;
            self.state[WIDTH - 1] += Constant::new(S::from(2));
        } else {
            self.state[WIDTH - 1] += Constant::ONE;
        }
    }
}

impl<
    'a,
    'b,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [LinearCombination<S>; WIDTH]>,
> Duplex<LinearCombination<S>> for DuplexImpl<'a, 'b, S, RATE, CAPACITY, WIDTH, P>
{
    fn reset(&mut self) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state.iter_mut().for_each(LinearCombination::clear);
    }

    fn absorb_native(&mut self, e: &LinearCombination<S>) {
        if self.phase == Phase::Squeeze {
            self.phase = Phase::Absorb;
            self.position = 0;
        } else if self.position == RATE {
            P::permute(self.circuit, &mut self.state);
            self.position = 0;
        }
        self.state[self.position] = e.clone();
        self.position += 1
    }

    fn squeeze_native(&mut self) -> LinearCombination<S> {
        if self.phase == Phase::Absorb {
            self.phase = Phase::Squeeze;
            self.pad();
            P::permute(self.circuit, &mut self.state);
            self.position = 0;
        } else if self.position == RATE {
            P::permute(self.circuit, &mut self.state);
            self.position = 0;
        }
        let e = self.state[self.position].clone();
        self.position += 1;
        e
    }
}

impl<
    'a,
    'b,
    S: Semiring + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<S, Domain = [LinearCombination<S>; WIDTH]>,
> UniformGenerator for DuplexImpl<'a, 'b, S, RATE, CAPACITY, WIDTH, P>
{
    type Output = LinearCombination<S>;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.squeeze_native()
    }
}
