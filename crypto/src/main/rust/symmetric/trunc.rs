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

use crate::algebra::AdditiveGroup;
use crate::symmetric::{CompressionFunction, Permutation};
use core::iter::zip;
use core::marker::PhantomData;

/// Trunc mode
///
/// <https://eprint.iacr.org/2026/1271>
pub struct Trunc<
    G: AdditiveGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [G; WIDTH]>,
> {
    phantom: PhantomData<P>,
}

impl<G: AdditiveGroup, const RANK: usize, const WIDTH: usize, P: Permutation<Domain = [G; WIDTH]>>
    Trunc<G, RANK, WIDTH, P>
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

impl<G: AdditiveGroup, const RANK: usize, const WIDTH: usize, P: Permutation<Domain = [G; WIDTH]>>
    Default for Trunc<G, RANK, WIDTH, P>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
    G: AdditiveGroup + Clone,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<Domain = [G; WIDTH]>,
> CompressionFunction for Trunc<G, RANK, WIDTH, P>
{
    type Hash = [G; RANK];

    fn compress(a: Self::Hash, b: Self::Hash) -> Self::Hash {
        let mut state = [G::ZERO; WIDTH];
        state[..WIDTH / 2].clone_from_slice(&a);
        state[WIDTH / 2..].clone_from_slice(&b);
        P::permute(&mut state);
        let mut hash = a;
        for (h, s) in zip(&mut hash, state) {
            *h += s
        }
        hash
    }
}
