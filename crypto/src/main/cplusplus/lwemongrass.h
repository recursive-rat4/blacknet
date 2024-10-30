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
    constexpr double SIGMA = 0.5;

    using Zq = FermatRing;

    using SecretKey = Matrix<Zq>;

    struct PublicKey {
        Matrix<Zq> a;
        Matrix<Zq> p;
    };

    struct CipherText {
        Vector<Zq> a;
        Vector<Zq> b;
    };

    static_assert(std::is_signed_v<typename Zq::NormType>);
    boost::random::uniform_int_distribution<typename Zq::NormType> bud(0, 1);
    boost::random::uniform_int_distribution<typename Zq::NormType> tud(-1, 1);
    DiscreteGaussianDistribution<typename Zq::NormType> dgd(0.0, SIGMA);

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
}

#endif
