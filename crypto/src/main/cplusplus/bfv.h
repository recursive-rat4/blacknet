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

#ifndef BLACKNET_CRYPTO_BFV_H
#define BLACKNET_CRYPTO_BFV_H

#include <type_traits>
#include <boost/random/uniform_int_distribution.hpp>

#include "discretegaussiandistribution.h"
#include "polynomialring.h"

// https://eprint.iacr.org/2012/144

template<typename Zt, typename Zq, std::size_t D>
struct BFV {
    // https://eprint.iacr.org/2019/939
    constexpr static const double SIGMA = 8.0 / std::sqrt(2.0 * std::numbers::pi);
    static_assert(SIGMA >= 3.19 && SIGMA <= 3.2);

    using Rt = CyclotomicRing<Zt, D>;
    using Rq = CyclotomicRing<Zq, D>;
    constexpr static const auto DELTA = Zq::modulus() / Zt::modulus();
    static_assert(std::is_signed_v<typename Zq::NormType>);

    using SecretKey = Rq;

    typedef struct {
        Rq a;
        Rq b;
    } PublicKey;

    typedef struct {
        Rq a;
        Rq b;
    } CipherText;

    using PlainText = Rt;

    boost::random::uniform_int_distribution<typename Zq::NormType> bud{0, 1};
    DiscreteGaussianDistribution<typename Zq::NormType> dgd{0.0, SIGMA};

    constexpr Rq lift(const Rt& rt) const {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i)
            rq.coefficients[i] = Zq(rt.coefficients[i].number());
        return rq;
    }

    template<typename RNG>
    SecretKey generateSecretKey(RNG& rng) {
        return Rq::random(rng, dgd);
    }

    template<typename RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto e = Rq::random(rng, dgd);
        auto a = Rq::random(rng);
        return { -(a * sk + e), a };
    }

    template<typename RNG>
    CipherText encrypt(RNG& rng, const PublicKey& pk, const PlainText& pt) {
        auto u = Rq::random(rng, dgd);
        auto e1 = Rq::random(rng, dgd);
        auto e2 = Rq::random(rng, dgd);
        return { pk.a * u + e1 + Zq(DELTA) * lift(pt), pk.b * u + e2 };
    }

    constexpr PlainText decrypt(const SecretKey& sk, const CipherText& ct) const {
        PlainText pt;
        auto d = ct.a + ct.b * sk;
        for (std::size_t i = 0; i < D; ++i) {
            pt.coefficients[i] = Zt(std::round(double(Zt::modulus()) * double(d.coefficients[i].number()) / double(Zq::modulus())));
        }
        return pt;
    }
};

#endif
