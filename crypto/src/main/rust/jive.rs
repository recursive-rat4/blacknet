/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::compressionfunction::CompressionFunction;
use crate::cyclicgroup::AdditiveCyclicGroup;
use crate::permutation::Permutation;
use core::marker::PhantomData;

/// Jive mode <https://eprint.iacr.org/2022/840>
pub struct Jive<
    G: AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [G; WIDTH]>,
> {
    phantom: PhantomData<P>,
}

impl<
    G: AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [G; WIDTH]>,
> Jive<G, RANK, WIDTH, P>
{
    pub const fn new() -> Self {
        const {
            assert!(RANK * 2 == WIDTH);
        }
        Self {
            phantom: PhantomData,
        }
    }
}

impl<
    G: AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [G; WIDTH]>,
> Default for Jive<G, RANK, WIDTH, P>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
    G: AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [G; WIDTH]>,
> CompressionFunction for Jive<G, RANK, WIDTH, P>
{
    type Hash = [G; RANK];

    fn compress(a: Self::Hash, b: Self::Hash) -> Self::Hash {
        let mut state = [G::IDENTITY; WIDTH];
        state[..WIDTH / 2].copy_from_slice(&a);
        state[WIDTH / 2..].copy_from_slice(&b);
        P::permute(&mut state);
        let mut hash = [G::IDENTITY; RANK];
        for i in 0..RANK {
            hash[i] = a[i] + b[i] + state[i] + state[i + RANK];
        }
        hash
    }
}
