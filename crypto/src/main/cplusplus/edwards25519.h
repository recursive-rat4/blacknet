/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_EDWARDS25519_H
#define BLACKNET_CRYPTO_EDWARDS25519_H

#include "bigint.h"
#include "bitint.h"
#include "primefield.h"
#include "twistededwardsgroupaffine.h"
#include "twistededwardsgroupextended.h"

namespace blacknet::crypto {

struct Field25519Params {
    constexpr static const std::size_t BITS = 255;
    constexpr static const UInt256 M = UInt256("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");
    constexpr static const UInt256 R2 = UInt256("00000000000000000000000000000000000000000000000000000000000005A4");
    constexpr static const UInt256 R3 = UInt256("000000000000000000000000000000000000000000000000000000000000D658");
    constexpr static const typename UInt256::L RN = UInt256::L(0x86BCA1AF286BCA1B);
    constexpr static const UInt256 TWO_INVERTED = UInt256("3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7");
    constexpr static const BitInt<254> P_MINUS_1_HALVED{"3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6"};
    constexpr static const BitInt<253> Q{"1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFB"};
    constexpr static const UInt256 S = UInt256(2);
    constexpr static const BitInt<252> Q_PLUS_1_HALVED{"0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE"};
    constexpr static const bool has_sparse_modulus = false;

    constexpr static UInt256 reduce(const UInt512& x) {
        // Montgomery reduction
        UInt512 tt(x);
        typename UInt256::LL c = 0;
        for (std::size_t i = 0; i < UInt256::LIMBS(); ++i) {
            typename UInt256::LL ll = 0;
            typename UInt256::L l = tt.limbs[i] * RN;
            for (std::size_t j = 0; j < UInt256::LIMBS(); ++j) {
                ll += UInt256::LL(l) * UInt256::LL(M.limbs[j]) + UInt256::LL(tt.limbs[i + j]);
                tt.limbs[i + j] = ll;
                ll >>= sizeof(typename UInt256::L) * 8;
            }
            c += UInt256::LL(tt.limbs[i + UInt256::LIMBS()]) + ll;
            tt.limbs[i + UInt256::LIMBS()] = c;
            c >>= sizeof(typename UInt256::L) * 8;
        }
        UInt256 t{tt.limbs[7], tt.limbs[6], tt.limbs[5], tt.limbs[4]};
        if (t >= M)
            t -= M;
        return t;
    }

    constexpr static UInt256 toForm(const UInt256& n) {
        return reduce(n * R2);
    }
    constexpr static UInt256 fromForm(const UInt256& n) {
        return reduce(UInt512(0, 0, 0, 0, n.limbs[3], n.limbs[2], n.limbs[1], n.limbs[0]));
    }
};

typedef PrimeField<Field25519Params> Field25519;

typedef TwistedEdwardsGroupAffine<
    Field25519,
    BitInt<255>,
    Field25519("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC"),
    Field25519("52036CEE2B6FFE738CC740797779E89800700A4D4141D8AB75EB4DCA135978A3")
> Edwards25519GroupAffine;

typedef TwistedEdwardsGroupExtended<
    Field25519,
    BitInt<255>,
    Field25519("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC"),
    Field25519("52036CEE2B6FFE738CC740797779E89800700A4D4141D8AB75EB4DCA135978A3")
> Edwards25519GroupExtended;

}

#endif
