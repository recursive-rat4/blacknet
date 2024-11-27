// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_PERVUSHIN_H
#define BLACKNET_CRYPTO_PERVUSHIN_H

#include "integerring.h"

struct PervushinRingParams {
    using I = int64_t;
    using L = __int128_t;
    using UI = uint64_t;
    using UL = __uint128_t;

    constexpr static const I M = 2305843009213693951;
    constexpr static const I R2 = 64;
    constexpr static const I RN = -2305843009213693953;
    constexpr static const I PROU = 2305843009213693950;
    constexpr static const std::size_t PROUD = 2;

    constexpr static I reduce(I x) {
        return (x & 2305843009213693951) + (x >> 61);
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 63) & 2305843009213693951);
    }
};

// 2⁶¹ - 1
typedef IntegerRing<PervushinRingParams> PervushinRing;

#endif
