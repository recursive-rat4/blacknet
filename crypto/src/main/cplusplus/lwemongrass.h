/*
 * Copyright (c) 2024 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_LWEMONGRASS_H
#define BLACKNET_CRYPTO_LWEMONGRASS_H

#include <optional>
#include <type_traits>
#include <boost/random/uniform_int_distribution.hpp>

#include "discretegaussiandistribution.h"
#include "fermat.h"
#include "matrix.h"
#include "vector.h"

/*
 * Snake-eye Resistance from LWE for Oblivious Message Retrieval and Robust Encryption
 * Zeyu Liu, Katerina Sotiraki, Eran Tromer, Yunhao Wang
 * August 19, 2024
 * https://eprint.iacr.org/2024/510
 */

namespace lwemongrass {
    constexpr std::size_t K = 1;
    constexpr std::size_t ELL = 3;
    constexpr std::size_t N1 = 936;
    constexpr std::size_t N2 = 760;
    constexpr std::size_t N = N1 - K;
    constexpr int R = 79;
    constexpr double SIGMA = 0.5;

    using Zq = FermatRing;
    constexpr int q_div_p = 32768;
    static_assert(std::is_signed_v<typename Zq::NormType>);

    using SecretKey = Matrix<Zq>;

    typedef struct {
        Matrix<Zq> a;
        Matrix<Zq> p;
    } PublicKey;

    typedef struct {
        Vector<Zq> a;
        Vector<Zq> b;
    } CipherText;

    using PlainText = Vector<Zq>;

    boost::random::uniform_int_distribution<typename Zq::NormType> bud(0, 1);
    boost::random::uniform_int_distribution<typename Zq::NormType> tud(-1, 1);
    DiscreteGaussianDistribution<typename Zq::NormType> dgd(0.0, SIGMA);

    constexpr bool isZeroK(const Vector<Zq>& v) {
        bool zero = true;
        for (std::size_t i = N; i < N1; ++i) {
            if (v[i] != Zq(0)) {
                zero = false;
                break;
            }
        }
        return zero;
    }

    template<typename RNG>
    SecretKey generateSecretKey(RNG& rng) {
        return (Matrix<Zq>::random(rng, tud, ELL, N) || Matrix<Zq>::random(rng, ELL, K)).transpose();
    }

    template<typename RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto e = Matrix<Zq>::random(rng, dgd, N2, ELL);
        auto a = Matrix<Zq>::random(rng, N2, N1);
        return { a, a * sk + e };
    }

    template<typename RNG>
    CipherText encrypt(RNG& rng, const PublicKey& pk, const PlainText& pt) {
        Vector<Zq> x;
        do {
            x = Vector<Zq>::random(rng, bud, N2);
        } while (isZeroK(x * pk.a));
        auto e1 = Vector<Zq>::random(rng, dgd, N) || Vector<Zq>(K, Zq(0));
        auto e2 = Vector<Zq>::random(rng, dgd, ELL);
        return { x * pk.a + e1, x * pk.p + e2 + pt * Zq(q_div_p) };
    }

    constexpr std::optional<PlainText> decrypt(const SecretKey& sk, const CipherText& ct) {
        if (isZeroK(ct.a))
            return std::nullopt;
        auto d = ct.a * sk - ct.b;
        auto pt = Vector<Zq>(ELL);
        for (std::size_t i = 0; i < ELL; ++i) {
            if (std::abs(d[i].number()) <= R)
                pt[i] = Zq(0);
            else if (q_div_p - std::abs(d[i].number()) <= R)
                pt[i] = Zq(1);
            else
                return std::nullopt;
        }
        return pt;
    }
}

#endif
