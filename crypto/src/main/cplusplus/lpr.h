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

#include <random>
#include <type_traits>

#include "convolution.h"
#include "discretegaussiandistribution.h"
#include "fermat.h"
#include "numbertheoretictransform.h"
#include "polynomialring.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2013/293

struct LPR {
    using Zt = FermatRing; // It would be logical to use `Z2` here, if optimizer were able to deal with it
    using Zq = FermatRing;

    constexpr static const std::size_t D = 1024;

    constexpr static const std::size_t H = 64;
    constexpr static const double SIGMA = 0.5;

    constexpr static const int DELTA = Zq::modulus() / 2;
    constexpr static const int HALF_DELTA = Zq::modulus() / 4;

    struct PlainTextRingParams {
        using Z = Zt;

        constexpr static const std::size_t N = D;

        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            convolution::negacyclic<Z, N>(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>&) {}
        constexpr static void fromForm(std::array<Z, N>&) {}
    };

    struct CipherTextRingParams {
        using Z = Zq;

        constexpr static const std::size_t N = D;

        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            convolution::negacyclic<Z, N>(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>&) {}
        constexpr static void fromForm(std::array<Z, N>&) {}
    };

    struct CipherTextNTTRingParams {
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

    using Rt = PolynomialRing<PlainTextRingParams>;
    using Rq = PolynomialRing<CipherTextRingParams>;
    using RqIso = PolynomialRing<CipherTextNTTRingParams>;

    constexpr static RqIso& isomorph(Rq&& f) {
        CipherTextNTTRingParams::toForm(f.coefficients);
        return reinterpret_cast<RqIso&>(f);
    }
    constexpr static Rq& isomorph(RqIso&& f) {
        CipherTextNTTRingParams::fromForm(f.coefficients);
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

    using PlainText = Rt;

    static_assert(std::is_signed_v<typename Zq::NumericType>);
    std::uniform_int_distribution<typename Zq::NumericType> tud{-1, 1};
    DiscreteGaussianDistribution<Zq> dgd{0.0, SIGMA};

    constexpr static Rq upscale(const Rt& rt) {
        Rq rq;
        for (std::size_t i = 0; i < D; ++i) {
            if (rt.coefficients[i] == Zt(0))
                rq.coefficients[i] = Zq(0);
            else
                rq.coefficients[i] = Zq(DELTA);
        }
        return rq;
    }

    template<std::uniform_random_bit_generator RNG>
    SecretKey generateSecretKey(RNG& rng) {
        return Rq::random(rng, tud, H);
    }

    template<std::uniform_random_bit_generator RNG>
    PublicKey generatePublicKey(RNG& rng, const SecretKey& sk) {
        auto e = Rq::random(rng, dgd);
        auto a = Rq::random(rng);
        return { -(a * sk + e), a };
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
            if (d.coefficients[i].absolute() <= HALF_DELTA)
                pt.coefficients[i] = Zt(0);
            else
                pt.coefficients[i] = Zt(1);
        }
        return pt;
    }
};

}

#endif
