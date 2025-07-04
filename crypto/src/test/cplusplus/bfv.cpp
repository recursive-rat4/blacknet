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

#include <boost/test/unit_test.hpp>

#include "bfv.h"
#include "convolution.h"
#include "fastrng.h"
#include "fermat.h"
#include "polynomialring.h"
#include "solinas62.h"

using namespace blacknet::crypto;

static FastDRG rng;

BOOST_AUTO_TEST_SUITE(BFVs)

struct RtParams {
    using Z = FermatRing;

    constexpr static const std::size_t N = 4;

    using Convolution = convolution::Negacyclic<Z, N>;
    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        Convolution::call(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};
using Rt = PolynomialRing<RtParams>;

struct RqParams {
    using Z = Solinas62Ring;

    constexpr static const std::size_t N = 4;

    using Convolution = convolution::Negacyclic<Z, N>;
    constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
        Convolution::call(r, a, b);
    }
    constexpr static void toForm(std::array<Z, N>&) {}
    constexpr static void fromForm(std::array<Z, N>&) {}
};
using Rq = PolynomialRing<RqParams>;

BOOST_AUTO_TEST_CASE(Gadgets) {
    using BFV = BFV<Rt, Rq>;
    BFV bfv;
    Rq a = Rq::random(rng);
    Rq b = Rq::random(rng);
    auto d = bfv.gadget_d(a);
    auto p = bfv.gadget_p(b);
    BOOST_TEST(a * b == d.dot(p), "Gadget Decomposition");
}

BOOST_AUTO_TEST_CASE(Tests) {
    using BFV = BFV<Rt, Rq>;
    BFV bfv;
    auto sk = bfv.generateSecretKey(rng);
    auto pk = bfv.generatePublicKey(rng, sk);
    BFV::PlainText pt{ 1, 2, 3, 4 };
    auto ct_sk = bfv.encrypt(rng, sk, pt);
    BOOST_TEST(pt == bfv.decrypt(sk, ct_sk), "Secret-key mode Decryption");
    auto ct_pk = bfv.encrypt(rng, pk, pt);
    BOOST_TEST(pt == bfv.decrypt(sk, ct_pk), "Public-key mode Decryption");

    BFV::Zt scalar{ 2 };
    BFV::PlainText pt1{ 2, };
    BFV::PlainText pt2{ 4, };
    auto ct1 = bfv.encrypt(rng, pk, pt1);
    auto ct2 = bfv.encrypt(rng, pk, pt1);

    BFV::Evaluator eval_add_pt{ ct1 };
    eval_add_pt += pt1;
    BOOST_TEST(pt2 == bfv.decrypt(sk, eval_add_pt.ct), "PlainText Addition");

    BFV::Evaluator eval_add_ct{ ct1 };
    eval_add_ct += ct2;
    BOOST_TEST(pt2 == bfv.decrypt(sk, eval_add_ct.ct), "CipherText Addition");

    BFV::Evaluator eval_mul_scalar{ ct1 };
    eval_mul_scalar *= scalar;
    BOOST_TEST(pt2 == bfv.decrypt(sk, eval_mul_scalar.ct), "Scalar Multiplication");

    BFV::Evaluator eval_mul_pt{ ct1 };
    eval_mul_pt *= pt1;
    BOOST_TEST(pt2 == bfv.decrypt(sk, eval_mul_pt.ct), "PlainText Multiplication");
}

BOOST_AUTO_TEST_SUITE_END()
