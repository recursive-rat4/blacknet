// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_FERMAT_H
#define BLACKNET_CRYPTO_FERMAT_H

#include "integerring.h"

struct FermatRingParams {
    using I = int32_t;
    using L = int64_t;
    using UI = uint32_t;
    using UL = uint64_t;

    constexpr static const I M = 65537;
    constexpr static const I R2 = 1;
    constexpr static const I RN = -65535;
    constexpr static const I PROU = 431;
    constexpr static const std::size_t PROUD = 1024;

    constexpr static I reduce(I x) {
        return (x & 0xFFFF) - (x >> 16);
    }
    constexpr static I freeze(I x) {
        return x;
    }
};

// 2¹⁶ + 1
typedef IntegerRing<FermatRingParams> FermatRing;

#endif
