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
use alloc::fmt::{Debug, Formatter, Result};
use alloc::vec;
use alloc::vec::Vec;

pub struct MerkleTree<F: CompressionFunction> {
    size: usize,
    nodes: Vec<F::Hash>,
}

impl<F: CompressionFunction<Hash: Clone>> MerkleTree<F> {
    pub fn compute_root(mut i: usize, leaf: F::Hash, branch: &Vec<F::Hash>) -> F::Hash {
        let mut hash = leaf;
        for node in branch {
            if (i & 1) == 1 {
                hash = F::compress(node.clone(), hash);
            } else {
                hash = F::compress(hash, node.clone());
            }
            i >>= 1;
        }
        hash
    }
}

impl<F: CompressionFunction<Hash: Clone + Default>> MerkleTree<F> {
    pub fn new(leaves: &[F::Hash]) -> Self {
        let size = leaves.len();
        if size != 0 {
            let mut nodes = Vec::<F::Hash>::with_capacity((size << 1) + 1);
            nodes.extend_from_slice(leaves);
            let mut j = 0;
            let mut l = size;
            while l > 1 {
                for i in (0..l).step_by(2) {
                    if i + 1 < l {
                        nodes.push(F::compress(nodes[i + j].clone(), nodes[i + j + 1].clone()));
                    } else {
                        nodes.push(F::compress(nodes[i + j].clone(), F::Hash::default()));
                    }
                }
                j += l;
                l = (l + 1) >> 1;
            }
            Self { size, nodes }
        } else {
            Self::default()
        }
    }

    pub fn branch(&self, mut i: usize) -> Vec<F::Hash> {
        let mut branch = Vec::<F::Hash>::with_capacity(bit_width(self.size) as usize);
        let mut j = 0;
        let mut l = self.size;
        while l > 1 {
            if (i ^ 1) < l {
                branch.push(self.nodes[(i ^ 1) + j].clone());
            } else {
                branch.push(F::Hash::default());
            }
            i >>= 1;
            j += l;
            l = (l + 1) >> 1;
        }
        branch
    }
}

impl<F: CompressionFunction> MerkleTree<F> {
    pub fn root(&self) -> &F::Hash {
        self.nodes.last().unwrap()
    }
}

impl<F: CompressionFunction<Hash: Clone>> Clone for MerkleTree<F> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            nodes: self.nodes.clone(),
        }
    }
}

impl<F: CompressionFunction<Hash: Default>> Default for MerkleTree<F> {
    fn default() -> Self {
        Self {
            size: 0,
            nodes: vec![F::Hash::default()],
        }
    }
}

impl<F: CompressionFunction<Hash: Debug>> Debug for MerkleTree<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.size, self.root())
    }
}

impl<F: CompressionFunction<Hash: PartialEq>> PartialEq for MerkleTree<F> {
    fn eq(&self, rps: &Self) -> bool {
        self.root() == rps.root()
    }
}

impl<F: CompressionFunction<Hash: Eq>> Eq for MerkleTree<F> {}

//RUST https://github.com/rust-lang/rust/issues/142326
const fn bit_width(n: usize) -> u32 {
    usize::BITS - n.leading_zeros()
}
