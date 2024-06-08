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
    VestaGroupProjective g(gx, gy, VestaField(1));
    VestaGroupProjective h(hx, hy, VestaField(1));
    PedersenCommitment<VestaGroupProjective> cs(g, h);
    VestaGroupProjective c1(c1x, c1y, VestaField(1));
    VestaGroupProjective c2(c2x, c2y, VestaField(1));
    BOOST_TEST(cs.open(c1, m1, r1), "Opening");
    BOOST_TEST(!cs.open(c2, m1, r1), "Binding");
    BOOST_TEST(!cs.open(c1, r1, m1), "Positional binding");
    BOOST_TEST(cs.open(c1 + c2, m1 + m2, r1 + r2), "Homomorphism");
}

BOOST_AUTO_TEST_SUITE_END()
