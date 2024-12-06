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

#include <numbers>
#include <type_traits>
#include <boost/random/uniform_int_distribution.hpp>

#include "discretegaussiandistribution.h"
#include "polynomialring.h"

// https://eprint.iacr.org/2012/144
// https://eprint.iacr.org/2024/1587

template<typename Rt, typename Rq>
struct BFV {
    using Zt = Rt::Z;
    using Zq = Rq::Z;
    constexpr static const std::size_t D = Rq::N;

    static_assert(Rt::N == Rq::N);
    static_assert(std::is_signed_v<typename Zt::NumericType>);
    static_assert(std::is_signed_v<typename Zq::NumericType>);

    // https://eprint.iacr.org/2019/939
    constexpr static const double SIGMA = 8.0 / std::sqrt(2.0 * std::numbers::pi);
    static_assert(SIGMA >= 3.19 && SIGMA <= 3.2);

    constexpr static const double DELTA = double(Zq::modulus()) / double(Zt::modulus());
    constexpr static const double INV_DELTA = double(Zt::modulus()) / double(Zq::modulus());

    using SecretKey = Rq;

    struct PublicKey {
        Rq a;
        Rq b;
    };

    struct CipherText {
        Rq a;
        Rq b;
    };

    using PlainText = Rt;

    struct Evaluator {
        CipherText ct;

        constexpr Evaluator& operator += (const PlainText& other) {
            ct.a += upscale(other);
            return *this;
        }

        constexpr Evaluator& operator += (const CipherText& other) {
            ct.a += other.a;
            ct.b += other.b;
            return *this;
        }

        constexpr Evaluator& operator *= (const PlainText& other) {
            Rq m(lift(other));
            ct.a *= m;
            ct.b *= m;
            return *this;
        }
    };

    boost::random::uniform_int_distribution<typename Zq::NumericType> bud{0, 1};
    DiscreteGaussianDistribution<typename Zq::NumericType> dgd{0.0, SIGMA};

    constexpr static Rq lift(const Rt& rt) {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i)
            rq.coefficients[i] = Zq(rt.coefficients[i].number());
        return rq;
    }

    constexpr static Rq upscale(const Rt& rt) {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i)
            rq.coefficients[i] = Zq(std::round(DELTA * rt.coefficients[i].number()));
        return rq;
    }

    template<typename RNG>
    SecretKey generateSecretKey(RNG& rng) {
        return Rq::random(rng, bud);
    }

    template<typename RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto e = Rq::random(rng, dgd);
        auto a = Rq::random(rng);
        return { -(a * sk + e), a };
    }

    template<typename RNG>
    CipherText encrypt(RNG& rng, const PublicKey& pk, const PlainText& pt) {
        auto u = Rq::random(rng, bud);
        auto e1 = Rq::random(rng, dgd);
        auto e2 = Rq::random(rng, dgd);
        return { pk.a * u + e1 + upscale(pt), pk.b * u + e2 };
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
