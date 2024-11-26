// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_DILITHIUMRING_H
#define BLACKNET_CRYPTO_DILITHIUMRING_H

#include "integerring.h"

// 2²³ - 2¹³ + 1
typedef IntegerRing<
    int32_t,
    int64_t,
    uint32_t,
    uint64_t,
    8380417,
    2365951,
    58728449,
    1753,
    512,
    [] (int32_t x) -> int32_t {
        int32_t t((x + (1 << 22)) >> 23);
        return x - t * 8380417;
    },
    [] (int32_t x) -> int32_t {
        return x + ((x >> 31) & 8380417);
    }
> DilithiumRing;

#endif
