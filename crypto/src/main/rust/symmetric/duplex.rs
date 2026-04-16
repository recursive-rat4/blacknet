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
use crate::random::UniformGenerator;
use crate::symmetric::Permutation;
use core::marker::PhantomData;
use zeroize::{DefaultIsZeroes, Zeroize};

/// The phase of sponge state
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Phase {
    /// Absorbing into sponge
    #[default]
    Absorb,
    /// Squeezing from sponge
    Squeeze,
}

impl DefaultIsZeroes for Phase {}

pub trait Duplexer: Sized + UniformGenerator {
    type Msg;

    fn reset(&mut self);

    fn absorb_msg(&mut self, e: Self::Msg);
    fn squeeze_msg(&mut self) -> Self::Msg;

    #[inline]
    fn absorb<S: Absorb<Self::Msg>>(&mut self, e: S) {
        e.absorb_into(self)
    }

    #[inline]
    fn absorb_iter<S: Absorb<Self::Msg>, I: Iterator<Item = S>>(&mut self, iter: I) {
        iter.for_each(|i| i.absorb_into(self));
    }

    #[inline]
    fn squeeze<S: Squeeze<Self::Msg>>(&mut self) -> S {
        S::squeeze_from(self)
    }

    #[inline]
    fn squeeze_with_size<S: SqueezeWithSize<Self::Msg>>(&mut self, size: usize) -> S {
        S::squeeze_from(self, size)
    }
}

pub trait Absorb<T> {
    fn absorb_into<D: Duplexer<Msg = T>>(self, duplex: &mut D);
}

pub trait Squeeze<T> {
    fn squeeze_from<D: Duplexer<Msg = T>>(duplex: &mut D) -> Self;
}

pub trait SqueezeWithSize<T> {
    fn squeeze_from<D: Duplexer<Msg = T>>(duplex: &mut D, size: usize) -> Self;
}

/// An implementation of the duplex construction.
///
/// Introduction: <https://keccak.team/files/CSF-0.1.pdf>
///
/// Overwrite mode: <https://eprint.iacr.org/2008/263>
///
/// Non-injective padding: <http://fuee.u-fukui.ac.jp/~hirose/publication/ask20160930.pdf>
///
#[derive(Zeroize)]
pub struct Duplex<
    S: AdditiveGroup + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> {
    phase: Phase,
    position: usize,
    state: [S; WIDTH],
    phantom: PhantomData<P>,
}

impl<
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Duplex<S, RATE, CAPACITY, WIDTH, P>
{
    pub const fn new() -> Self {
        const {
            assert!(RATE + CAPACITY == WIDTH);
        }
        Self {
            phase: Phase::Absorb,
            position: 0,
            state: [S::ZERO; WIDTH],
            phantom: PhantomData,
        }
    }

    pub fn with_iv(iv: &[S; CAPACITY]) -> Self {
        let mut duplex = Self::new();
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

    pub fn sneak(self) -> [S; WIDTH] {
        self.state
    }
}

impl<
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Clone for Duplex<S, RATE, CAPACITY, WIDTH, P>
{
    fn clone(&self) -> Self {
        Self {
            phase: self.phase,
            position: self.position,
            state: self.state.clone(),
            phantom: PhantomData,
        }
    }
}

impl<
    S: AdditiveGroup + Copy + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Copy for Duplex<S, RATE, CAPACITY, WIDTH, P>
{
}

impl<
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Default for Duplex<S, RATE, CAPACITY, WIDTH, P>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Duplexer for Duplex<S, RATE, CAPACITY, WIDTH, P>
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
            P::permute(&mut self.state);
            self.position = 0;
        }
        self.state[self.position] = e;
        self.position += 1
    }

    fn squeeze_msg(&mut self) -> S {
        if self.phase == Phase::Absorb {
            self.phase = Phase::Squeeze;
            self.pad();
            P::permute(&mut self.state);
            self.position = 0;
        } else if self.position == RATE {
            P::permute(&mut self.state);
            self.position = 0;
        }
        let e = self.state[self.position].clone();
        self.position += 1;
        e
    }
}

impl<
    S: AdditiveGroup + Clone + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> UniformGenerator for Duplex<S, RATE, CAPACITY, WIDTH, P>
{
    type Output = S;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.squeeze_msg()
    }
}
