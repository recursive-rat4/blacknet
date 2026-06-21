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
use crate::assigner::assigment::Assigment;
use crate::assigner::symmetric::{CompressionFunction, Permutation};
use core::iter::zip;
use core::marker::PhantomData;

pub struct Trunc<
    'a,
    G: AdditiveGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [G; WIDTH]>,
> {
    phantom: PhantomData<P>,
    assigment: &'a Assigment<G>,
}

impl<
    'a,
    G: AdditiveGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [G; WIDTH]>,
> Trunc<'a, G, RANK, WIDTH, P>
{
    pub const fn new(assigment: &'a Assigment<G>) -> Self {
        const {
            assert!(RANK * 2 == WIDTH);
        }
        Self {
            phantom: PhantomData,
            assigment,
        }
    }
}

impl<
    'a,
    G: AdditiveGroup + Clone,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [G; WIDTH]>,
> CompressionFunction for Trunc<'a, G, RANK, WIDTH, P>
{
    type Hash = [G; RANK];

    fn compress(&self, a: Self::Hash, b: Self::Hash) -> Self::Hash {
        let mut state = [G::ZERO; WIDTH];
        state[..WIDTH / 2].clone_from_slice(&a);
        state[WIDTH / 2..].clone_from_slice(&b);
        P::permute(self.assigment, &mut state);
        let mut hash = a;
        for (h, s) in zip(&mut hash, state) {
            *h += s
        }
        hash
    }
}
