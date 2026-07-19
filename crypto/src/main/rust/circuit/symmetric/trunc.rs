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

use crate::algebra::{AdditiveGroup, Semiring};
use crate::circuit::builder::{CircuitBuilder, LinearCombination};
use crate::circuit::symmetric::{CompressionFunction, Permutation};
use core::marker::PhantomData;

pub struct Trunc<
    'a,
    'b,
    G: Semiring + AdditiveGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [LinearCombination<G>; WIDTH]>,
> {
    circuit: &'a CircuitBuilder<'b, G>,
    phantom: PhantomData<P>,
}

impl<
    'a,
    'b,
    G: Semiring + AdditiveGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [LinearCombination<G>; WIDTH]>,
> Trunc<'a, 'b, G, RANK, WIDTH, P>
{
    pub const fn new(circuit: &'a CircuitBuilder<'b, G>) -> Self {
        const {
            assert!(RANK * 2 == WIDTH);
        }
        Self {
            circuit,
            phantom: PhantomData,
        }
    }
}

impl<
    'a,
    'b,
    G: Semiring + AdditiveGroup + Clone,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [LinearCombination<G>; WIDTH]>,
> CompressionFunction for Trunc<'a, 'b, G, RANK, WIDTH, P>
{
    type Hash = [LinearCombination<G>; RANK];

    fn compress(&self, a: &Self::Hash, b: &Self::Hash) -> Self::Hash {
        let mut state = [LinearCombination::<G>::ZERO; WIDTH];
        state[..WIDTH / 2].clone_from_slice(a);
        state[WIDTH / 2..].clone_from_slice(b);
        P::permute(self.circuit, &mut state);
        let mut hash = [LinearCombination::<G>::ZERO; RANK];
        for i in 0..RANK {
            hash[i] = &a[i] + &state[i];
        }
        hash
    }
}
