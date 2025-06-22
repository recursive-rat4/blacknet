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

#ifndef BLACKNET_CRYPTO_BFV_H
#define BLACKNET_CRYPTO_BFV_H

#include <algorithm>
#include <bit>
#include <cmath>
#include <numbers>
#include <random>
#include <type_traits>

#include "discretegaussiandistribution.h"
#include "latticegadget.h"
#include "polynomialring.h"
#include "vector.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2012/144
// https://eprint.iacr.org/2024/1587

template<typename Rt, typename Rq>
struct BFV {
    using Zt = Rt::BaseRing;
    using Zq = Rq::BaseRing;
    constexpr static const std::size_t D = Rq::dimension();

    static_assert(Rt::dimension() == Rq::dimension());
    static_assert(std::is_signed_v<typename Zt::NumericType>);
    static_assert(std::is_signed_v<typename Zq::NumericType>);

    constexpr static const std::size_t H = std::min<std::size_t>(256, D);

    // https://eprint.iacr.org/2019/939
    constexpr static const double SIGMA = 3.191538243211461;

    constexpr static const double DELTA = double(Zq::modulus()) / double(Zt::modulus());
    constexpr static const double INV_DELTA = double(Zt::modulus()) / double(Zq::modulus());

    const uint64_t ELL = 5;
    const uint64_t OMEGA = std::bit_ceil(uint64_t(std::pow(Zq::modulus(), 1.0 / ELL)));

    using SecretKey = Rq;

    struct PublicKey {
        Rq a;
        Rq b;
    };

    struct EvaluationKey {
        Vector<Rq> square;
        Vector<Rq> sigma;
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

        constexpr Evaluator& operator *= (const Zt& other) {
            Zq m(lift(other));
            ct.a *= m;
            ct.b *= m;
            return *this;
        }

        constexpr Evaluator& operator *= (const PlainText& other) {
            Rq m(lift(other));
            ct.a *= m;
            ct.b *= m;
            return *this;
        }
    };

    std::uniform_int_distribution<typename Zq::NumericType> tud{-1, 1};
    DiscreteGaussianDistribution<Zq> dgd{0.0, SIGMA};

    constexpr static Zq lift(const Zt& zt) {
        return Zq(zt.balanced());
    }

    constexpr static Rq lift(const Rt& rt) {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i)
            rq.coefficients[i] = lift(rt.coefficients[i]);
        return rq;
    }

    constexpr static Rq upscale(const Rt& rt) {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i)
            rq.coefficients[i] = Zq(std::round(DELTA * rt.coefficients[i].balanced()));
        return rq;
    }

    constexpr Vector<Rq> gadget_d(const Rq& rq) const {
        return LatticeGadget<Rq>::decompose(OMEGA, ELL, rq);
    }

    constexpr Vector<Rq> gadget_p(const Rq& rq) const {
        return LatticeGadget<Rq>::vector(OMEGA, ELL, rq);
    }

    template<std::uniform_random_bit_generator RNG>
    SecretKey generateSecretKey(RNG& rng) {
        return Rq::random(rng, tud, H);
    }

    template<std::uniform_random_bit_generator RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto a = Rq::random(rng);
        auto e = Rq::random(rng, dgd);
        return { -(a * sk + e), a };
    }

    template<std::uniform_random_bit_generator RNG>
    CipherText encrypt(RNG& rng, const SecretKey& sk, const PlainText& pt) {
        auto a = Rq::random(rng);
        auto e = Rq::random(rng, dgd);
        return { a * sk + e + upscale(pt), -a };
    }

    template<std::uniform_random_bit_generator RNG>
    CipherText encrypt(RNG& rng, const PublicKey& pk, const PlainText& pt) {
        auto u = generateSecretKey(rng);
        auto e1 = Rq::random(rng, dgd);
        auto e2 = Rq::random(rng, dgd);
        return { pk.a * u + e1 + upscale(pt), pk.b * u + e2 };
    }

    constexpr PlainText decrypt(const SecretKey& sk, const CipherText& ct) const {
        PlainText pt;
        auto d = ct.a + ct.b * sk;
        for (std::size_t i = 0; i < D; ++i) {
            pt.coefficients[i] = Zt(std::round(INV_DELTA * d.coefficients[i].balanced()));
        }
        return pt;
    }
};

}

#endif
