/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use bytemuck::Zeroable;
use core::{iter::zip, mem::MaybeUninit};

/// Generator of uniformly distributed values.
pub trait UniformGenerator {
    /// The type of generated values.
    type Output;

    /// Generate a single value.
    fn generate(&mut self) -> Self::Output;

    /// Generate a sequence of values.
    fn fill(&mut self, sequence: &mut [Self::Output]) {
        for i in sequence {
            *i = self.generate()
        }
    }

    /// Discard `n` output values.
    fn discard(&mut self, n: u32) {
        for _ in 0..n {
            let _ = self.generate();
        }
    }
}

/// Generator of uniformly distributed bytes.
pub trait UniformBitGenerator: UniformGenerator<Output = u8> {}

impl<G: UniformGenerator<Output = u8>> UniformBitGenerator for G {}

pub trait Seedable: Sized {
    type Seed;

    fn from_seed(seed: &Self::Seed) -> Self;

    fn reseed(&mut self, seed: &Self::Seed) {
        *self = Self::from_seed(seed);
    }
}

/// Wrapper for a generator of arrays to output elements one by one.
#[derive(Clone, Copy, Zeroable)]
pub struct BufferedGenerator<G, T, const N: usize>
where
    G: UniformGenerator<Output = [T; N]>,
    T: Copy,
{
    position: usize,
    buffer: [MaybeUninit<T>; N],
    generator: G,
}

impl<G: UniformGenerator<Output = [T; N]>, T: Copy, const N: usize> BufferedGenerator<G, T, N> {
    /// Construct a new generator with default state.
    pub fn new() -> Self
    where
        G: Default,
    {
        Self::with_generator(G::default())
    }

    /// Wrap a generator.
    pub const fn with_generator(generator: G) -> Self {
        const {
            assert!(N > 0);
        }
        Self {
            position: N,
            buffer: [const { MaybeUninit::<T>::uninit() }; N],
            generator,
        }
    }
}

impl<G: UniformGenerator<Output = [T; N]> + Default, T: Copy, const N: usize> Default
    for BufferedGenerator<G, T, N>
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<G: UniformGenerator<Output = [T; N]>, T: Copy, const N: usize> UniformGenerator
    for BufferedGenerator<G, T, N>
{
    type Output = T;

    fn generate(&mut self) -> Self::Output {
        if self.position == N {
            for (l, r) in zip(&mut self.buffer, self.generator.generate()) {
                l.write(r);
            }
            self.position = 0;
        }
        let output = unsafe { self.buffer.get_unchecked(self.position).assume_init_read() };
        self.position += 1;
        output
    }

    fn discard(&mut self, n: u32) {
        let pos_n = self.position + n as usize;
        if pos_n <= N {
            self.position = pos_n;
            return;
        }
        let (q, r) = (pos_n / N, pos_n % N);
        self.generator.discard(q as u32 - 1);
        self.position = r;
        for (l, r) in zip(&mut self.buffer, self.generator.generate()) {
            l.write(r);
        }
    }
}

impl<G: UniformGenerator<Output = [T; N]> + Seedable, T: Copy, const N: usize> Seedable
    for BufferedGenerator<G, T, N>
{
    type Seed = G::Seed;

    fn from_seed(seed: &Self::Seed) -> Self {
        let generator = G::from_seed(seed);
        Self::with_generator(generator)
    }

    fn reseed(&mut self, seed: &Self::Seed) {
        self.position = N;
        self.generator.reseed(seed);
    }
}
