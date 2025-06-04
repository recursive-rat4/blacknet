/*
 * Copyright (c) 2024-2025 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef BLACKNET_CRYPTO_BLACKLEMON_H
#define BLACKNET_CRYPTO_BLACKLEMON_H

#include <optional>
#include <random>

#include "lpr.h"

namespace blacknet::crypto {

struct BlackLemon {
    using PKE = LPR;
    constexpr static const std::size_t ELL = 2;
    constexpr static const int R = 40;

    using Zq = PKE::Zq;
    using Rq = PKE::Rq;

    struct SecretKey {
        PKE::SecretKey a;
        Rq b;
    };

    struct PublicKey {
        PKE::PublicKey a;
        Rq b;
    };

    using CipherText = PKE::CipherText;

    using PlainText = PKE::PlainText;

    PKE pke;

    template<std::uniform_random_bit_generator RNG>
    SecretKey generateSecretKey(RNG& rng) {
        auto a = pke.generateSecretKey(rng);
        auto b = Rq::random(rng);
        return { a, b };
    }

    template<std::uniform_random_bit_generator RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto a = pke.generatePublicKey(rng, sk.a);
        auto b = -sk.b;
        return { a, b };
    }

    template<std::uniform_random_bit_generator RNG>
    CipherText encrypt(RNG& rng, const PublicKey& pk, const PlainText& pt) {
        auto ct = pke.encrypt(rng, pk.a, pt);
        ct.a += pk.b;
        return ct;
    }

    constexpr PlainText decrypt(const SecretKey& sk, const CipherText& ct) {
        return pke.decrypt(sk.a, { ct.a + sk.b, ct.b });
    }

    constexpr std::optional<PlainText> detect(const SecretKey& sk, const CipherText& ct) {
        PlainText pt;
        auto d = ct.a + ct.b * sk.a + sk.b;
        for (std::size_t i = 0; i < Rq::dimension(); ++i) {
            if (d.coefficients[i].absolute() <= R)
                pt.coefficients[i] = Zq(0);
            else if (PKE::DELTA - d.coefficients[i].absolute() <= R)
                pt.coefficients[i] = Zq(1);
            else
                return std::nullopt;
        }
        for (std::size_t i = 0; i < ELL; ++i) {
            if (pt.coefficients[i] != Zq(0))
                return std::nullopt;
        }
        return pt;
    }
};

}

#endif
