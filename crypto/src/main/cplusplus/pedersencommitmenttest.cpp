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

#include <boost/test/unit_test.hpp>
#include <vector>

#include "pastacurves.h"
#include "pedersencommitment.h"

BOOST_AUTO_TEST_SUITE(PedersenCommitments)

BOOST_AUTO_TEST_CASE(single) {
    VestaField  gx; std::istringstream("33074014122d93a8ac69e0bbc472768ebf2760c0e53f73abf0e395d8b1b5b478") >> gx;
    VestaField  gy; std::istringstream("3604f572d11bae3cccf8a6895d8e06b3c7388e54a5acda9f5e62d33a72bbc566") >> gy;
    VestaField  hx; std::istringstream("245a92dbb72f4e95e0be3595344d0bc58978c7b7c9c1a5b2128d9d7eb3d6328a") >> hx;
    VestaField  hy; std::istringstream("11bac7e68bd74ee7a7a43f6b1f9e206e8b8ac7c8d2bae596ef891c301155ad1e") >> hy;
    VestaField c1x; std::istringstream("3e8cadd38b46b13201817a1aee9717d725593b85200de9a1e0d17d9360e6b861") >> c1x;
    VestaField c1y; std::istringstream("0bf3b36d73b5f244cff3a65e8e8130cfacfa79fb1c3cd0404f5bac1b50b5778d") >> c1y;
    VestaField c2x; std::istringstream("0462e663bdd1b93aff1bf6c6aa7ef8e71521ddc1494e4727a9baf78b87946eef") >> c2x;
    VestaField c2y; std::istringstream("3342441f4969d6bff28fe055db320b90794c17a419b102c56ad8179c9a76459e") >> c2y;
    PallasField m1; std::istringstream("09e21902c37d0c6dc4c1c8143faefa86a192cac72bdc0d89828a2d1ce3d813b3") >> m1;
    PallasField m2; std::istringstream("374bb94b3a48c4cadbc80878bf5082692a25001e84865cbd73f3f0cb7308bc72") >> m2;
    PallasField r1; std::istringstream("1ab0bd7178dbc83ec8ec11aa0bf46e5cae406812d865fa9a96beccac98aa0f5d") >> r1;
    PallasField r2; std::istringstream("10af23b9642c311b7b270d22fd0cb8efbcdee017d8d25246dedeb7bf06064906") >> r2;
    VestaGroupJacobian g(gx, gy, VestaField(1));
    VestaGroupJacobian h(hx, hy, VestaField(1));
    PedersenCommitment<VestaGroupJacobian> cs({g, h});
    VestaGroupJacobian c1(c1x, c1y, VestaField(1));
    VestaGroupJacobian c2(c2x, c2y, VestaField(1));
    BOOST_TEST(cs.open(c1, m1, r1), "Opening");
    BOOST_TEST(!cs.open(c2, m1, r1), "Binding");
    BOOST_TEST(!cs.open(c1, r1, m1), "Positional binding");
    BOOST_TEST(cs.open(c1 + c2, m1 + m2, r1 + r2), "Homomorphism");
}

BOOST_AUTO_TEST_CASE(vector) {
    VestaField p1x; std::istringstream("33074014122d93a8ac69e0bbc472768ebf2760c0e53f73abf0e395d8b1b5b478") >> p1x;
    VestaField p1y; std::istringstream("3604f572d11bae3cccf8a6895d8e06b3c7388e54a5acda9f5e62d33a72bbc566") >> p1y;
    VestaField p2x; std::istringstream("245a92dbb72f4e95e0be3595344d0bc58978c7b7c9c1a5b2128d9d7eb3d6328a") >> p2x;
    VestaField p2y; std::istringstream("11bac7e68bd74ee7a7a43f6b1f9e206e8b8ac7c8d2bae596ef891c301155ad1e") >> p2y;
    VestaField p3x; std::istringstream("09ac10faca63a9a711abf2b4a585e3cf2e88f388191286c0304ae69b5530187a") >> p3x;
    VestaField p3y; std::istringstream("1837afd5380b84650dcfead81a5db502dd73c5d6ba9d380037e0c9fb1bfaa6e7") >> p3y;
    VestaField p4x; std::istringstream("12e3fe4c5fdd7d837434f551ee12f28dd62764674747bcc633fc2f2378bf8434") >> p4x;
    VestaField p4y; std::istringstream("02603a78ee085e0e6ec1b7fd06b819fdb0acd38c5b51977a21b0575b59d37c96") >> p4y;
    VestaField c1x; std::istringstream("2a76c98cb6dc763cb83510d7facba0ed1193ba380c7644acc57e424e626944ca") >> c1x;
    VestaField c1y; std::istringstream("3f46c0cb9513a17154b8cb0e9291b415cefc07e6c7d67dc4ddb5c8786f40f179") >> c1y;
    VestaField c2x; std::istringstream("397d1ed03abfd518ff0753644cc7a514a74feb7eea316d080333b21e9fd9fe81") >> c2x;
    VestaField c2y; std::istringstream("0a4db0584ecaebadc6581658ec61e7ed4c41cbd172ca2ae861a83985de51afbf") >> c2y;
    PallasField m1; std::istringstream("09e21902c37d0c6dc4c1c8143faefa86a192cac72bdc0d89828a2d1ce3d813b3") >> m1;
    PallasField m2; std::istringstream("374bb94b3a48c4cadbc80878bf5082692a25001e84865cbd73f3f0cb7308bc72") >> m2;
    PallasField m3; std::istringstream("1ab0bd7178dbc83ec8ec11aa0bf46e5cae406812d865fa9a96beccac98aa0f5d") >> m3;
    PallasField m4; std::istringstream("10af23b9642c311b7b270d22fd0cb8efbcdee017d8d25246dedeb7bf06064906") >> m4;
    PallasField m5; std::istringstream("24e02c656c29446963355b2375536270b270d8d7bd72ca3b4784eba7b8e46ce1") >> m5;
    std::vector v1{m1, m2, m3, m4};
    std::vector v2{m1, m2, m3, m5};
    std::vector v3{m1, m3, m2, m4};
    VestaGroupJacobian p1(p1x, p1y, VestaField(1));
    VestaGroupJacobian p2(p2x, p2y, VestaField(1));
    VestaGroupJacobian p3(p3x, p3y, VestaField(1));
    VestaGroupJacobian p4(p4x, p4y, VestaField(1));
    PedersenCommitment<VestaGroupJacobian> cs({p1, p2, p3, p4});
    VestaGroupJacobian c1(c1x, c1y, VestaField(1));
    VestaGroupJacobian c2(c2x, c2y, VestaField(1));
    BOOST_TEST(cs.open(c1, v1), "Opening");
    BOOST_TEST(!cs.open(c1, v2), "Binding");
    BOOST_TEST(!cs.open(c1, v3), "Positional binding");
    BOOST_TEST(cs.open(c1 + c2, {m1 + m1, m2 + m2, m3 + m3, m4 + m5}), "Homomorphism");
}

BOOST_AUTO_TEST_SUITE_END()
