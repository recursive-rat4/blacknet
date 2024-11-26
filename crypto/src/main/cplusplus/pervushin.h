// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_PERVUSHIN_H
#define BLACKNET_CRYPTO_PERVUSHIN_H

#include "integerring.h"

// 2⁶¹ - 1
typedef IntegerRing<
    int64_t,
    __int128_t,
    uint64_t,
    __uint128_t,
    2305843009213693951,
    64,
    -2305843009213693953,
    2305843009213693950,
    2,
    [] (int64_t x) -> int64_t {
        return (x & 2305843009213693951) + (x >> 61);
    },
    [] (int64_t x) -> int64_t {
        return x + ((x >> 63) & 2305843009213693951);
    }
> PervushinRing;

#endif
