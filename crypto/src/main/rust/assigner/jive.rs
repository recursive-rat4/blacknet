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

use crate::algebra::AdditiveCyclicGroup;
use crate::algebra::Presemiring;
use crate::assigner::assigment::Assigment;
use crate::assigner::compressionfunction::CompressionFunction;
use crate::assigner::permutation::Permutation;
use core::marker::PhantomData;

pub struct Jive<
    'a,
    G: Presemiring + AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [G; WIDTH]>,
> {
    phantom: PhantomData<P>,
    assigment: &'a Assigment<G>,
}

impl<
    'a,
    G: Presemiring + AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [G; WIDTH]>,
> Jive<'a, G, RANK, WIDTH, P>
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
    G: Presemiring + AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [G; WIDTH]>,
> CompressionFunction for Jive<'a, G, RANK, WIDTH, P>
{
    type Hash = [G; RANK];

    fn compress(&self, a: Self::Hash, b: Self::Hash) -> Self::Hash {
        let mut state = [G::ZERO; WIDTH];
        state[..WIDTH / 2].copy_from_slice(&a);
        state[WIDTH / 2..].copy_from_slice(&b);
        P::permute(self.assigment, &mut state);
        let mut hash = [G::ZERO; RANK];
        for i in 0..RANK {
            hash[i] = a[i] + b[i] + state[i] + state[i + RANK];
        }
        hash
    }
}
