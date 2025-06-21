// Auto-generated with rings.sage

#ifndef BLACKNET_CRYPTO_LM62_H
#define BLACKNET_CRYPTO_LM62_H

#include <array>

#include "integerring.h"

namespace blacknet::crypto {

struct LM62RingParams {
    using I = int64_t;
    using L = __int128_t;
    using UI = uint64_t;
    using UL = __uint128_t;

    constexpr static const bool is_division_ring = true;

    constexpr static const std::size_t BITS = 62;
    constexpr static const I M = 2900318160026599457;
    constexpr static const I R2 = 1882224917555981558;
    constexpr static const I RN = -629412166104022047;
    constexpr static const std::array<I, 16> TWIDDLES = {1044835113549954874, 569218631026025462, -784935944205243435, 572552586692549161, -361369180022199466, 178484251486083901, -114684091461287305, -1101467716902666349, 747721600660698373, -505064029198444890, -1050328543640198522, -69461906194906578, 102519286186593439, -1172228903700312769, 1182002436770437611, 1070319519127119695};
    constexpr static const I INVERSE_TWIDDLES = 1152921504606846976;

    constexpr static I reduce(I x) {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * 2900318160026599457;
    }
};

// 2⁶¹ + 2⁵⁹ + 2⁵⁴ + 2⁵ + 1
typedef IntegerRing<LM62RingParams> LM62Ring;

}

#endif
