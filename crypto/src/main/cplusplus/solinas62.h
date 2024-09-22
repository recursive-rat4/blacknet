// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_SOLINAS62_H
#define BLACKNET_CRYPTO_SOLINAS62_H

#include <array>
#include <optional>

#include "integerring.h"

namespace blacknet::crypto {

struct Solinas62RingParams {
    using I = int64_t;
    using L = __int128_t;
    using UI = uint64_t;
    using UL = __uint128_t;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t BITS = 62;
    constexpr static const I M = 4611686018427387617;
    constexpr static const I R2 = 1317904;
    constexpr static const I RN = -3454747365720865503;
    constexpr static const std::array<I, 16> TWIDDLES = {1148, 1909850507366759230, 1925695278238998043, -1196916019830996660, -990812595794006616, -750689347914722583, 25775166106861705, 147213721977915467, -769555794185807974, -500579210516158037, -1816761452615928001, 1463977254396149782, -1276567290860230089, -802780904022862126, 500001058544660596, -1174682222540695415};
    constexpr static const I INVERSE_TWIDDLES = 1152921504606846976;
    constexpr static const std::optional<I> two_inverted = 574;

    constexpr static I reduce(I x) {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * 4611686018427387617;
    }
};

// 2⁶² - 2⁸ - 2⁵ + 1
typedef IntegerRing<Solinas62RingParams> Solinas62Ring;

}

#endif
