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

#include <boost/test/unit_test.hpp>

#include "jive.h"
#include "merkletree.h"
#include "poseidon2pervushin.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(MerkleTrees)

using Jive = Poseidon2PervushinJive;
using Hash = Jive::Hash;
constexpr static Hash null{0,0,0,0};
using MerkleTree = MerkleTree<Jive, null>;

BOOST_AUTO_TEST_CASE(empty) {
    MerkleTree tree1;
    std::vector<Hash> leaves;
    MerkleTree tree2(std::move(leaves));

    BOOST_TEST(null == tree1.root());
    BOOST_TEST(tree1 == tree2);
}

BOOST_AUTO_TEST_CASE(even) {
    Hash h1{0,0,0,1};
    Hash h2{0,0,0,2};
    std::vector<Hash> leaves{ h1, h2 };
    MerkleTree tree(leaves);
    std::vector<Hash> b1{h2};
    std::vector<Hash> b2{h1};

    BOOST_TEST(tree.root() == Jive::compress(h1, h2));
    BOOST_TEST(b2 == tree.branch(1));

    BOOST_TEST(tree.root() != MerkleTree::root(1, h1, b1));
    BOOST_TEST(tree.root() != MerkleTree::root(0, h2, b1));
    BOOST_TEST(tree.root() != MerkleTree::root(0, h1, b2));
}

BOOST_AUTO_TEST_CASE(odd) {
    std::vector<Hash> leaves{
        Hash{0,0,0,1},
        Hash{0,0,0,2},
        Hash{0,0,0,3},
        Hash{0,0,0,4},
        Hash{0,0,0,5},
    };
    MerkleTree tree(leaves);

    for (std::size_t i = 0; i < leaves.size(); ++i) {
        const Hash& leaf = leaves[i];
        std::vector<Hash> branch = tree.branch(i);
        BOOST_TEST(tree.root() == MerkleTree::root(i, leaf, branch));
    }
}

BOOST_AUTO_TEST_SUITE_END()
