// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_SOLINAS62_H
#define BLACKNET_CRYPTO_SOLINAS62_H

#include "integerring.h"

// 2⁶² - 2⁸ - 2⁵ + 1
typedef IntegerRing<
    int64_t,
    __int128_t,
    uint64_t,
    __uint128_t,
    4611686018427387617,
    1317904,
    -3454747365720865503,
    71638321025703109,
    32,
    [] (int64_t x) -> int64_t {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * 4611686018427387617;
    },
    [] (int64_t x) -> int64_t {
        return x + ((x >> 63) & 4611686018427387617);
    }
> Solinas62Ring;

#endif
