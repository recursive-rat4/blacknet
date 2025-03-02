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

#ifndef BLACKNET_CRYPTO_MERKLE_TREE_H
#define BLACKNET_CRYPTO_MERKLE_TREE_H

#include <bit>
#include <iostream>
#include <vector>

template<
    typename Function,
    Function::Hash null
>
struct MerkleTree {
    using Hash = Function::Hash;

    std::size_t size;
    std::vector<Hash> nodes;

    constexpr static void compute(std::size_t size, std::vector<Hash>& nodes) {
        if (size > 0) {
            nodes.reserve((size << 1) + 1);
            std::size_t j = 0;
            for (std::size_t l = size; l > 1; l = (l + 1) >> 1) {
                for (std::size_t i = 0; i < l; i += 2) {
                    if (i + 1 < l) {
                        nodes.emplace_back(Function::compress(nodes[i + j], nodes[i + j + 1]));
                    } else {
                        nodes.emplace_back(Function::compress(nodes[i + j], null));
                    }
                }
                j += l;
            }
        } else {
            nodes.reserve(1);
            nodes.emplace_back(null);
        }
    }

    constexpr MerkleTree()
        : size(0), nodes() { compute(size, nodes); }
    constexpr MerkleTree(const std::vector<Hash>& leaves)
        : size(leaves.size()), nodes(leaves) { compute(size, nodes); }
    constexpr MerkleTree(std::vector<Hash>&& leaves)
        : size(leaves.size()), nodes(std::move(leaves)) { compute(size, nodes); }
    constexpr MerkleTree(MerkleTree&& other) noexcept
        : size(other.size), nodes(std::move(other.nodes)) {}

    constexpr MerkleTree& operator = (MerkleTree&& other) noexcept {
        size = other.size;
        nodes = std::move(other.nodes);
        return *this;
    }

    constexpr bool operator == (const MerkleTree& other) const {
        return root() == other.root();
    }

    constexpr const Hash& root() const {
        return nodes.back();
    }

    constexpr std::vector<Hash> branch(std::size_t i) const {
        std::vector<Hash> branch;
        branch.reserve(std::bit_width(size));
        std::size_t j = 0;
        for (std::size_t l = size; l > 1; l = (l + 1) >> 1) {
            if ((i ^ 1) < l) {
                branch.push_back(nodes[(i ^ 1) + j]);
            } else {
                branch.emplace_back(null);
            }
            i >>= 1;
            j += l;
        }
        return branch;
    }

    constexpr static Hash root(std::size_t i, const Hash& leaf, const std::vector<Hash>& branch) {
        Hash hash(leaf);
        for (const Hash& node : branch) {
            if (i & 1) {
                hash = Function::compress(node, hash);
            } else {
                hash = Function::compress(hash, node);
            }
            i >>= 1;
        }
        return hash;
    }

    friend std::ostream& operator << (std::ostream& out, const MerkleTree& val)
    {
        return out << '(' << val.size << ", " << val.root() << ')';
    }
};

#endif
