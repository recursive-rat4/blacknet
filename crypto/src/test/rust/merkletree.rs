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

use blacknet_crypto::compressionfunction::CompressionFunction;

type Z = blacknet_crypto::pervushin::PervushinField;
type Jive = blacknet_crypto::poseidon2pervushin::JivePoseidon2Pervushin;
type Hash = <Jive as CompressionFunction>::Hash;
type MerkleTree = blacknet_crypto::merkletree::MerkleTree<Jive>;

#[test]
fn empty() {
    let null: Hash = [0, 0, 0, 0].map(Z::from);

    let tree1 = MerkleTree::default();
    let leaves = Vec::<Hash>::new();
    let tree2 = MerkleTree::new(&leaves);

    assert_eq!(tree1.root(), &null);
    assert_eq!(tree2, tree1);
}

#[test]
fn even() {
    let h1: Hash = [0, 0, 0, 1].map(Z::from);
    let h2: Hash = [0, 0, 0, 2].map(Z::from);
    let leaves = vec![h1, h2];
    let tree = MerkleTree::new(&leaves);
    let b1 = vec![h2];
    let b2 = vec![h1];

    assert_eq!(tree.root(), &Jive::compress(h1, h2));
    assert_eq!(tree.branch(1), b2);

    assert_ne!(&MerkleTree::compute_root(1, h1, &b1), tree.root());
    assert_ne!(&MerkleTree::compute_root(0, h2, &b1), tree.root());
    assert_ne!(&MerkleTree::compute_root(0, h1, &b2), tree.root());
}

#[test]
fn odd() {
    let leaves = [
        [0, 0, 0, 1].map(Z::from),
        [0, 0, 0, 2].map(Z::from),
        [0, 0, 0, 3].map(Z::from),
        [0, 0, 0, 4].map(Z::from),
        [0, 0, 0, 5].map(Z::from),
    ];
    let tree = MerkleTree::new(&leaves);

    for (i, leaf) in leaves.into_iter().enumerate() {
        let branch = tree.branch(i);
        assert_eq!(&MerkleTree::compute_root(i, leaf, &branch), tree.root());
    }
}
