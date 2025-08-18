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

use crate::distribution::UniformGenerator;
use crate::group::AdditiveGroup;
use crate::permutation::Permutation;
use core::array;
use core::marker::PhantomData;

// https://keccak.team/files/CSF-0.1.pdf
// https://eprint.iacr.org/2008/263
// http://fuee.u-fukui.ac.jp/~hirose/publication/ask20160930.pdf

#[derive(Clone, Copy, Eq, PartialEq)]
enum Phase {
    Absorb,
    Squeeze,
}

pub trait Duplex<T: Copy>: Default + UniformGenerator {
    fn reset(&mut self);

    fn absorb_native(&mut self, e: T);
    fn squeeze_native(&mut self) -> T;

    #[inline]
    fn absorb<S: Absorb<T>>(&mut self, e: &S) {
        e.absorb_into(self)
    }

    #[inline]
    fn squeeze<S: Squeeze<T>>(&mut self) -> S {
        S::squeeze_from(self)
    }

    #[inline]
    fn squeeze_with_size<S: SqueezeWithSize<T>>(&mut self, size: usize) -> S {
        S::squeeze_from(self, size)
    }
}

pub trait Absorb<T: Copy> {
    fn absorb_into(&self, duplex: &mut impl Duplex<T>);
}

impl<T: Copy> Absorb<T> for T {
    #[inline]
    fn absorb_into(&self, duplex: &mut impl Duplex<T>) {
        duplex.absorb_native(*self)
    }
}

impl<T: Copy + Absorb<T>, const N: usize> Absorb<T> for [T; N] {
    #[inline]
    fn absorb_into(&self, duplex: &mut impl Duplex<T>) {
        self.iter().for_each(|i| duplex.absorb(i))
    }
}

impl<T: Copy + Absorb<T>> Absorb<T> for Vec<T> {
    #[inline]
    fn absorb_into(&self, duplex: &mut impl Duplex<T>) {
        self.iter().for_each(|i| duplex.absorb(i))
    }
}

pub trait Squeeze<T: Copy> {
    fn squeeze_from(duplex: &mut impl Duplex<T>) -> Self;
}

impl<T: Copy> Squeeze<T> for T {
    #[inline]
    fn squeeze_from(duplex: &mut impl Duplex<T>) -> Self {
        duplex.squeeze_native()
    }
}

impl<T: Copy, const N: usize> Squeeze<T> for [T; N] {
    #[inline]
    fn squeeze_from(duplex: &mut impl Duplex<T>) -> Self {
        array::from_fn(|_| duplex.squeeze())
    }
}

pub trait SqueezeWithSize<T: Copy> {
    fn squeeze_from(duplex: &mut impl Duplex<T>, size: usize) -> Self;
}

impl<T: Copy + Squeeze<T>> SqueezeWithSize<T> for Vec<T> {
    #[inline]
    fn squeeze_from(duplex: &mut impl Duplex<T>, size: usize) -> Self {
        (0..size).map(|_| duplex.squeeze()).collect::<Vec<T>>()
    }
}

#[derive(Clone, Copy)]
pub struct DuplexImpl<
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
    S: AdditiveGroup + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> DuplexImpl<S, RATE, CAPACITY, WIDTH, P>
{
    pub fn with_iv(iv: &[S; CAPACITY]) -> Self {
        let mut duplex = Self::default();
        duplex.state[RATE..WIDTH].copy_from_slice(iv);
        duplex
    }

    pub fn reset_with_iv(&mut self, iv: &[S; CAPACITY]) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state[..RATE].fill(S::IDENTITY);
        self.state[RATE..WIDTH].copy_from_slice(iv);
    }

    fn pad(&mut self) {
        if self.position != RATE {
            self.state[self.position] = S::from(1);
            self.position += 1;
            self.state[self.position..RATE].fill(S::IDENTITY);
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
    S: AdditiveGroup + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Default for DuplexImpl<S, RATE, CAPACITY, WIDTH, P>
{
    fn default() -> Self {
        Self {
            phase: Phase::Absorb,
            position: 0,
            state: [S::IDENTITY; WIDTH],
            phantom: PhantomData,
        }
    }
}

impl<
    S: AdditiveGroup + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> Duplex<S> for DuplexImpl<S, RATE, CAPACITY, WIDTH, P>
{
    fn reset(&mut self) {
        self.phase = Phase::Absorb;
        self.position = 0;
        self.state = [S::IDENTITY; WIDTH];
    }

    fn absorb_native(&mut self, e: S) {
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

    fn squeeze_native(&mut self) -> S {
        if self.phase == Phase::Absorb {
            self.phase = Phase::Squeeze;
            self.pad();
            P::permute(&mut self.state);
            self.position = 0;
        } else if self.position == RATE {
            P::permute(&mut self.state);
            self.position = 0;
        }
        let e = self.state[self.position];
        self.position += 1;
        e
    }
}

impl<
    S: AdditiveGroup + From<i8>,
    const RATE: usize,
    const CAPACITY: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [S; WIDTH]>,
> UniformGenerator for DuplexImpl<S, RATE, CAPACITY, WIDTH, P>
{
    type Output = S;

    #[inline]
    fn generate(&mut self) -> Self::Output {
        self.squeeze_native()
    }
}
