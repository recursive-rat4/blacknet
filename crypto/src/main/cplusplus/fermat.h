// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_FERMAT_H
#define BLACKNET_CRYPTO_FERMAT_H

#include "integerring.h"

// 2¹⁶ + 1
typedef IntegerRing<
    int32_t,
    int64_t,
    uint32_t,
    uint64_t,
    65537,
    1,
    -65535,
    431,
    1024,
    [] (int32_t x) -> int32_t {
        return (x & 0xFFFF) - (x >> 16);
    },
    [] (int32_t x) -> int32_t {
        return x;
    }
> FermatRing;

#endif
