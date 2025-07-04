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

#include "convolution.h"
#include "discretegaussiandistribution.h"
#include "fermat.h"
#include "numbertheoretictransform.h"
#include "polynomialring.h"
#include "polynomialringntt.h"
#include "ternaryuniformdistribution.h"
#include "z2.h"

namespace blacknet::crypto {

// https://eprint.iacr.org/2013/293

struct LPR {
    using Zt = Z2;
    using Zq = FermatRing;

    constexpr static const std::size_t D = 1024;

    constexpr static const std::size_t H = 64;
    constexpr static const double SIGMA = 0.5;

    constexpr static const int DELTA = Zq::modulus() / 2;
    constexpr static const int HALF_DELTA = Zq::modulus() / 4;

    struct PlainTextRingParams {
        using Z = Zt;

        constexpr static const std::size_t N = D;

        using Convolution = convolution::Negacyclic<Z, N>;
        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            Convolution::call(r, a, b);
        }
    };

    struct CipherTextRingParams {
        using Z = Zq;

        constexpr static const std::size_t N = D;

        using Convolution = convolution::Negacyclic<Z, N>;
        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            Convolution::call(r, a, b);
        }
    };

    struct CipherTextNTTRingParams {
        using Isomorphism = CipherTextRingParams;
        using Z = Zq;

        constexpr static const std::size_t N = D;

        using Convolution = NTT<Z, N>::Convolution;
        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            Convolution::call(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>& a) {
            NTT<Z, N>::cooley_tukey(a);
        }
        constexpr static void fromForm(std::array<Z, N>& a) {
            NTT<Z, N>::gentleman_sande(a);
        }
    };

    using Rt = PolynomialRing<PlainTextRingParams>;
    using Rq = PolynomialRing<CipherTextRingParams>;
    using RqIso = PolynomialRingNTT<CipherTextNTTRingParams>;

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
        TernaryUniformDistribution<Zq, RNG> tud;
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
