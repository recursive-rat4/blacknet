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

#ifndef BLACKNET_CRYPTO_LPR_H
#define BLACKNET_CRYPTO_LPR_H

#include <type_traits>
#include <boost/random/uniform_int_distribution.hpp>

#include "convolution.h"
#include "discretegaussiandistribution.h"
#include "fermat.h"
#include "numbertheoretictransform.h"
#include "polynomialring.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2013/293

struct LPR {
    using Zq = FermatRing;
    static_assert(std::is_signed_v<typename Zq::NumericType>);

    constexpr static const std::size_t D = 1024;

    constexpr static const std::size_t H = 64;
    constexpr static const double SIGMA = 0.5;

    constexpr static const int DELTA = Zq::modulus() / 2;
    constexpr static const int HALF_DELTA = Zq::modulus() / 4;

    struct CanonicalRingParams {
        using Z = Zq;

        constexpr static const std::size_t N = D;

        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            convolution::negacyclic<Z, N>(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>&) {}
        constexpr static void fromForm(std::array<Z, N>&) {}
    };

    struct NTTRingParams {
        using Z = Zq;

        constexpr static const std::size_t N = D;

        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            ntt::convolute<Z, N>(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>& a) {
            ntt::cooley_tukey<Z, N>(a);
        }
        constexpr static void fromForm(std::array<Z, N>& a) {
            ntt::gentleman_sande<Z, N>(a);
        }
    };

    using Rq = PolynomialRing<CanonicalRingParams>;
    using RqIso = PolynomialRing<NTTRingParams>;

    constexpr static RqIso& isomorph(Rq&& f) {
        NTTRingParams::toForm(f.coefficients);
        return reinterpret_cast<RqIso&>(f);
    }
    constexpr static Rq& isomorph(RqIso&& f) {
        NTTRingParams::fromForm(f.coefficients);
        return reinterpret_cast<Rq&>(f);
    }

    using SecretKey = Rq;

    struct PublicKey {
        Rq a;
        Rq b;
    };

    struct CipherText {
        Rq a;
        Rq b;
    };

    using PlainText = Rq;

    boost::random::uniform_int_distribution<typename Zq::NumericType> tud{-1, 1};
    DiscreteGaussianDistribution<typename Zq::NumericType> dgd{0.0, SIGMA};

    constexpr static Rq upscale(const Rq& rt) {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i) {
            if (rt.coefficients[i] == Zq(0))
                rq.coefficients[i] = Zq(0);
            else
                rq.coefficients[i] = Zq(DELTA);
        }
        return rq;
    }

    template<typename RNG>
    SecretKey generateSecretKey(RNG& rng) {
        return Rq::random(rng, tud, H);
    }

    template<typename RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto e = Rq::random(rng, dgd);
        auto a = Rq::random(rng);
        return { -(a * sk + e), a };
    }

    template<typename RNG>
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
            if (std::abs(d.coefficients[i].number()) <= HALF_DELTA)
                pt.coefficients[i] = Zq(0);
            else
                pt.coefficients[i] = Zq(1);
        }
        return pt;
    }
};

}

#endif
