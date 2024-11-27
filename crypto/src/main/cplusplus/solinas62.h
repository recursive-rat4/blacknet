// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_SOLINAS62_H
#define BLACKNET_CRYPTO_SOLINAS62_H

#include "integerring.h"

struct Solinas62RingParams {
    using I = int64_t;
    using L = __int128_t;
    using UI = uint64_t;
    using UL = __uint128_t;

    constexpr static const I M = 4611686018427387617;
    constexpr static const I R2 = 1317904;
    constexpr static const I RN = -3454747365720865503;
    constexpr static const I PROU = 71638321025703109;
    constexpr static const std::size_t PROUD = 32;

    constexpr static I reduce(I x) {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * 4611686018427387617;
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 63) & 4611686018427387617);
    }
};

// 2⁶² - 2⁸ - 2⁵ + 1
typedef IntegerRing<Solinas62RingParams> Solinas62Ring;

#endif
