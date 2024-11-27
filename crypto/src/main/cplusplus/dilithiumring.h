// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_DILITHIUMRING_H
#define BLACKNET_CRYPTO_DILITHIUMRING_H

#include "integerring.h"

struct DilithiumRingParams {
    using I = int32_t;
    using L = int64_t;
    using UI = uint32_t;
    using UL = uint64_t;

    constexpr static const I M = 8380417;
    constexpr static const I R2 = 2365951;
    constexpr static const I RN = 58728449;
    constexpr static const I PROU = 1753;
    constexpr static const std::size_t PROUD = 512;

    constexpr static I reduce(I x) {
        int32_t t((x + (1 << 22)) >> 23);
        return x - t * 8380417;
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 31) & 8380417);
    }
};

// 2²³ - 2¹³ + 1
typedef IntegerRing<DilithiumRingParams> DilithiumRing;

#endif
