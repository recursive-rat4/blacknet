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
use crate::algebra::Semiring;
use crate::circuit::builder::{CircuitBuilder, LinearCombination};
use crate::circuit::compressionfunction::CompressionFunction;
use crate::circuit::permutation::Permutation;
use core::array;
use core::marker::PhantomData;

pub struct Jive<
    'a,
    'b,
    G: Semiring + AdditiveCyclicGroup,
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
    G: Semiring + AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [LinearCombination<G>; WIDTH]>,
> Jive<'a, 'b, G, RANK, WIDTH, P>
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
    G: Semiring + AdditiveCyclicGroup,
    const RANK: usize,
    const WIDTH: usize,
    P: Permutation<G, Domain = [LinearCombination<G>; WIDTH]>,
> CompressionFunction for Jive<'a, 'b, G, RANK, WIDTH, P>
{
    type Hash = [LinearCombination<G>; RANK];

    fn compress(&self, a: &Self::Hash, b: &Self::Hash) -> Self::Hash {
        let mut state: [LinearCombination<G>; WIDTH] = array::from_fn(|_| LinearCombination::new());
        state[..WIDTH / 2].clone_from_slice(a);
        state[WIDTH / 2..].clone_from_slice(b);
        P::permute(self.circuit, &mut state);
        let mut hash: [LinearCombination<G>; RANK] = array::from_fn(|_| LinearCombination::new());
        for i in 0..RANK {
            hash[i] = &a[i] + &b[i] + &state[i] + &state[i + RANK];
        }
        hash
    }
}
