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

BOOST_AUTO_TEST_SUITE(PastaCurves)

BOOST_AUTO_TEST_CASE(fieldAdd) {
    UInt256     a; std::istringstream("2c5a3233336a186012edd7a62943cf0ae38a93b9454d5791b9825d4531fbf11c") >> a;
    UInt256     b; std::istringstream("34a99c1d1ad68aeb1d35bcf74ddb040b86ba0a05331200ef3e995b42c73be34a") >> b;
    PallasField c; std::istringstream("2103ce504e40a34b3023949d771ed31647fe04c26f125f655eee879af937d465") >> c;
    VestaField  d; std::istringstream("2103ce504e40a34b3023949d771ed31647fe04c26ecaafa36bd4cd66f937d465") >> d;
    BOOST_TEST(c == PallasField(a) + PallasField(b));
    BOOST_TEST(d == VestaField(a) + VestaField(b));
    BOOST_TEST(c == PallasField(0) + c);
    BOOST_TEST(d == d + VestaField(0));
    BOOST_TEST(PallasField(1) == PallasField(1) + PallasField(0));
    BOOST_TEST(VestaField(1) == VestaField(0) + VestaField(1));
}

BOOST_AUTO_TEST_CASE(fieldMul) {
    UInt256     a; std::istringstream("11640cdb3d3a126dabde403009808a4cae45ec00ffac7480d80ac9142abb607f") >> a;
    UInt256     b; std::istringstream("0a5111b1ee7f41260df2a030fc99d5aa095ae34332a190ba7ca6d9b54a5d1c85") >> b;
    PallasField c; std::istringstream("0b5842e91b2c5b9b253f653330dcf9d57d1d745479140a959684c13a5a25b6e6") >> c;
    VestaField  d; std::istringstream("0158030f7f4f7138ea54d0e0a8797e99ee4c3526ef9c67ccede788174b1f2172") >> d;
    BOOST_TEST(c == PallasField(a) * PallasField(b));
    BOOST_TEST(d == VestaField(a) * VestaField(b));
    BOOST_TEST(PallasField(0) == PallasField(0) * c);
    BOOST_TEST(VestaField(0) == d * VestaField(0));
    BOOST_TEST(c == c * PallasField(1));
    BOOST_TEST(d == VestaField(1) * d);
}

BOOST_AUTO_TEST_CASE(fieldSub) {
    UInt256     a; std::istringstream("063c6fa6bc7df187ee00659a73a97b1589892a4ae753fe00c7b3764ddd663cd2") >> a;
    UInt256     b; std::istringstream("20ac2a42b38f940e1bdc81e7b258588c04aee2f11a782e579033601a00df0730") >> b;
    PallasField c; std::istringstream("2590456408ee5d79d223e3b2c1512289a720e055d628c8c4d0ad4720dc8735a3") >> c;
    VestaField  d; std::istringstream("2590456408ee5d79d223e3b2c1512289a720e055d6707886c3c70154dc8735a3") >> d;
    BOOST_TEST(c == PallasField(a) - PallasField(b));
    BOOST_TEST(d == VestaField(a) - VestaField(b));
    BOOST_TEST(c == c - PallasField(0));
    BOOST_TEST(d == d - VestaField(0));
    BOOST_TEST(PallasField(0) == PallasField(1) - PallasField(1));
    BOOST_TEST(VestaField(0) == VestaField(1) - VestaField(1));
}

BOOST_AUTO_TEST_CASE(fieldDiv) {
    UInt256     a; std::istringstream("3faced132f5641f57b1162d06ed827d8ca9fa69f0c7b14822818eef4db6f6fdc") >> a;
    UInt256     b; std::istringstream("152d43a9a19991aa7f8c98ed185a79eda9b2562e4c456bb554c0c0d4d0362904") >> b;
    PallasField c; std::istringstream("3112d3dbd9cb47dd10c20edd49686b9713d5160fb2560360acc84d06bada7442") >> c;
    VestaField  d; std::istringstream("0e1fd01ec64fffe6a6fc237d1608308ddaa1efcb579ea243a347caaf8778061c") >> d;
    BOOST_TEST(c == PallasField(a) / PallasField(b));
    BOOST_TEST(d == VestaField(a) / VestaField(b));
    BOOST_TEST(PallasField(0) == PallasField(0) / c);
    BOOST_CHECK_THROW(d / VestaField(0), ArithmeticException);
    BOOST_TEST(PallasField(1) == PallasField(1) / PallasField(1));
    BOOST_TEST(d == d / VestaField(1));
}

BOOST_AUTO_TEST_CASE(fieldNeg) {
    UInt256     a; std::istringstream("12610bc44a0bbc319a91fc24e99a98ef2bd29a2b535bbd1a74bc100a698e34fa") >> a;
    PallasField b; std::istringstream("2d9ef43bb5f443ce656e03db16656710f673fed0b5f13c01247120e29671cb07") >> b;
    VestaField  c; std::istringstream("2d9ef43bb5f443ce656e03db16656710f673fed0b638ebc3178adb169671cb07") >> c;
    BOOST_TEST(b == -PallasField(a));
    BOOST_TEST(c == -VestaField(a));
    BOOST_TEST(PallasField(0) == -PallasField(0));
    BOOST_TEST(VestaField(0) == -VestaField(0));
    BOOST_TEST(PallasField(1) == -(-PallasField(1)));
    BOOST_TEST(VestaField(1) == -(-VestaField(1)));
}

BOOST_AUTO_TEST_CASE(fieldSquare) {
    PallasField a; std::istringstream("2f4564953a3b3bf9fffa19e805dfcd1b1b8381501d83664a5203d7cafa95c2ad") >> a;
    PallasField b; std::istringstream("2e4f0f106b3a0c9948816bf44d2587f755014bcbfb7150a2030c0f3eb82402b1") >> b;
    BOOST_TEST(b == a.square());
    BOOST_TEST(VestaField(0) == VestaField(0).square());
    BOOST_TEST(VestaField(1) == VestaField(1).square());
}

BOOST_AUTO_TEST_CASE(fieldInv) {
    PallasField a; std::istringstream("0f34fe2fd15703dc7eba4a68d48fa9ee0e9ab8746f759eb8fc23828a4aa48900") >> a;
    PallasField b; std::istringstream("087f2909b3c53a656a9f0f126b8458afa89ececeb5676d93c9d4594c4aacc34d") >> b;
    BOOST_TEST(a == b.invert());
    BOOST_TEST(b == a.invert());
    BOOST_CHECK_THROW(VestaField(0).invert(), ArithmeticException);
}

BOOST_AUTO_TEST_CASE(fieldSqrt) {
    UInt256     a; std::istringstream("35aeb661a5f2e7df341a8f256036c025e07b8e45002f7d9da0c8f7b5aa744aea") >> a;
    PallasField b; std::istringstream("344a642baaa8f21985d0757617709370cdc5b87574ecd97b4cf3c9d915689609") >> b;
    UInt256     c; std::istringstream("39fce7dbf35569b5dc603860e3254bf9e61e3b57ba958a05a121b318906fe126") >> c;
    VestaField  d; std::istringstream("2fd1206ca31cb1de80ffb18d6b4e5095edafca2beb056dfe0125bf1e0cae890a") >> d;
    BOOST_TEST(b == *PallasField(a).sqrt());
    BOOST_TEST(!VestaField(a).sqrt());
    BOOST_TEST(d == *VestaField(c).sqrt());
    BOOST_TEST(!PallasField(c).sqrt());
    BOOST_TEST(PallasField(0) == *PallasField(0).sqrt());
    BOOST_TEST(VestaField(0) == *VestaField(0).sqrt());
    BOOST_TEST(PallasField(1) == *PallasField(1).sqrt());
    BOOST_TEST(VestaField(1) == *VestaField(1).sqrt());
}

BOOST_AUTO_TEST_CASE(groupNegAffine) {
    PallasField ax; std::istringstream("2c998f5cd6f89a5323244238dcb0e122f3c48b690d17895d64c622fe7b134872") >> ax;
    PallasField ay; std::istringstream("2376d983140e67283c34cb1b20d3a6889b55892b51c224c059ba1f97a768959b") >> ay;
    PallasField bx; std::istringstream("2c998f5cd6f89a5323244238dcb0e122f3c48b690d17895d64c622fe7b134872") >> bx;
    PallasField by; std::istringstream("1c89267cebf198d7c3cb34e4df2c597786f10fd0b78ad45b3f73115558976a66") >> by;
    VestaField  cx; std::istringstream("2b84f575fc91b8f506713c696425fd86ea1f134bdb0f2821816f00ab1eeeaa92") >> cx;
    VestaField  cy; std::istringstream("179180e8abc3d15ed1d6bc287b7debe66b7c386cad750458ad956514255556bd") >> cy;
    VestaField  dx; std::istringstream("2b84f575fc91b8f506713c696425fd86ea1f134bdb0f2821816f00ab1eeeaa92") >> dx;
    VestaField  dy; std::istringstream("286e7f17543c2ea12e2943d784821419b6ca608f5c1fa484deb1860cdaaaa944") >> dy;
    PallasGroupAffine a(ax, ay);
    PallasGroupAffine b(bx, by);
    VestaGroupAffine  c(cx, cy);
    VestaGroupAffine  d(dx, dy);
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(PallasGroupAffine() == -PallasGroupAffine());
    BOOST_TEST(VestaGroupAffine() == -VestaGroupAffine());
}

BOOST_AUTO_TEST_CASE(groupAddAffine) {
    PallasField ax; std::istringstream("1e3dbd8ef7121f586a32c8789be6c1bd516ea0b7b5e00d356527f3b9137c7f13") >> ax;
    PallasField ay; std::istringstream("0c09c8b193a30e6989afa1cd8e3f468529cc2294b5111c80dc53080d10a133e3") >> ay;
    PallasField bx; std::istringstream("172c422e616dc9017cb392143dcdb133e1071d8e87806ccd9b222d82665aac69") >> bx;
    PallasField by; std::istringstream("0fb0e51efc9e8cd9c0a70e8fa507ec59fcb5da21d8cac79550c4f98d1dc2d362") >> by;
    PallasField cx; std::istringstream("3105fd2e4cf209b0db4e0e0772661ffaee9083b4e5faac71251d9ddbf05c2f23") >> cx;
    PallasField cy; std::istringstream("067e082d0d17fffdd4de37c218a55e188dbb09200621dad577fab3b592cf9ef4") >> cy;
    VestaField  dx; std::istringstream("3d3b0ea90d13082aa6862f0dac1e211c286614f222bafe7210862d448ef0e466") >> dx;
    VestaField  dy; std::istringstream("2b63efb469e111e71293b98fbe5008688cb8de0ca571a0075ea200e74abca6f9") >> dy;
    VestaField  ex; std::istringstream("1f85aa11a81f4464c19b28e5c55ace5b51689ef25f63156cce7d59e28969a639") >> ex;
    VestaField  ey; std::istringstream("3816248bb82336b770bc06e56883e8fa92c4557f4b16f1ab9fbd831db7750df8") >> ey;
    VestaField  fx; std::istringstream("1f85aa11a81f4464c19b28e5c55ace5b51689ef25f63156cce7d59e28969a639") >> fx;
    VestaField  fy; std::istringstream("07e9db7447dcc9488f43f91a977c17058f82437cbe7db731ec896803488af209") >> fy;
    PallasGroupAffine a(ax, ay);
    PallasGroupAffine b(bx, by);
    PallasGroupAffine c(cx, cy);
    VestaGroupAffine  d(dx, dy);
    VestaGroupAffine  e(ex, ey);
    VestaGroupAffine  f(fx, fy);
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(e == d + d);
    BOOST_TEST(VestaGroupAffine() == e + f);
    BOOST_TEST(c == PallasGroupAffine() + c);
    BOOST_TEST(c == c + PallasGroupAffine());
    BOOST_TEST(VestaGroupAffine() == VestaGroupAffine() + VestaGroupAffine());
}

BOOST_AUTO_TEST_CASE(groupMulAffine) {
    PallasField ax; std::istringstream("3aed134ed42ad34f18db7529fb0ed4470dbb0a157d676eca74f7789208b87676") >> ax;
    PallasField ay; std::istringstream("2a7a1566f8a07fe9bc87e23a8556e2e144afbe659053d2bfcbbaaa5a42ed856b") >> ay;
    PallasField cx; std::istringstream("2a0da0b30d7ff6d2956f3eeb2f72dc75310b85f70aa9123640ed78f1b6c3ff03") >> cx;
    PallasField cy; std::istringstream("2ddbebbf3c0412bc46ffaec08aaebc3c6bd717f3205bb841814983d016f79ec0") >> cy;
    PallasGroupAffine a(ax, ay);
    VestaField   b; std::istringstream("0e18ddb951f8a3a10c33028e6cd15a9b4480c3c825f515b6da24b75e7c813623") >> b;
    PallasGroupAffine c(cx, cy);
    PallasField  d; std::istringstream("251d364ed569cbf14184665ce3fa321e9678002959e04609d1a0ecc692cee9e1") >> d;
    BOOST_TEST(c == a * b);
    BOOST_TEST(a == a * VestaField(1));
    BOOST_TEST(PallasGroupAffine() == a * VestaField(0));
    BOOST_TEST(VestaGroupAffine() == VestaGroupAffine() * d);
}

BOOST_AUTO_TEST_CASE(groupNegProjective) {
    PallasField ax; std::istringstream("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5") >> ax;
    PallasField ay; std::istringstream("2f89c29d9ae36f7c0f20ef5d73f85cea5fdc1cfeae3b96e36c377d3b2f1afb4d") >> ay;
    PallasField bx; std::istringstream("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5") >> bx;
    PallasField by; std::istringstream("10763d62651c9083f0df10a28c07a315c26a7bfd5b1162382cf5b3b1d0e504b4") >> by;
    VestaField  cx; std::istringstream("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17") >> cx;
    VestaField  cy; std::istringstream("0e2e3683b3e12f2b986560a0b3a208f29066185aad807056b440e687f990a70a") >> cy;
    VestaField  dx; std::istringstream("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17") >> dx;
    VestaField  dy; std::istringstream("31d1c97c4c1ed0d4679a9f5f4c5df70d91e080a15c143886d8060499066f58f7") >> dy;
    PallasGroupProjective a(ax, ay, PallasField(1));
    PallasGroupProjective b(bx, by, PallasField(1));
    VestaGroupProjective  c(cx, cy, VestaField(1));
    VestaGroupProjective  d(dx, dy, VestaField(1));
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(PallasGroupProjective() == -PallasGroupProjective());
    BOOST_TEST(VestaGroupProjective() == -VestaGroupProjective());
}

BOOST_AUTO_TEST_CASE(groupAddProjective) {
    PallasField ax; std::istringstream("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c") >> ax;
    PallasField ay; std::istringstream("01a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15") >> ay;
    PallasField bx; std::istringstream("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c") >> bx;
    PallasField by; std::istringstream("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba") >> by;
    PallasField cx; std::istringstream("0201da427944269dee8b83e3cb8400f980a26ca9b89e6787e97c70ab09460d2e") >> cx;
    PallasField cy; std::istringstream("1d7929dcd5888af7651396fbcf1c145e178f5cbbbc9f497496c9b531692df787") >> cy;
    VestaField  dx; std::istringstream("2e3f99264efffdf2e6a620de2fd553baadc50da215ba7d2cace02a1843cab60e") >> dx;
    VestaField  dy; std::istringstream("3076516f0a8d132db8e5d71e15f1455c39b6cffa67946cd15b5daeb331557ba4") >> dy;
    VestaField  ex; std::istringstream("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8") >> ex;
    VestaField  ey; std::istringstream("01d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0") >> ey;
    VestaField  fx; std::istringstream("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8") >> fx;
    VestaField  fy; std::istringstream("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61") >> fy;
    PallasGroupProjective a(ax, ay, PallasField(1));
    PallasGroupProjective b(bx, by, PallasField(1));
    PallasGroupProjective c(cx, cy, PallasField(1));
    VestaGroupProjective  d(dx, dy, VestaField(1));
    VestaGroupProjective  e(ex, ey, VestaField(1));
    VestaGroupProjective  f(fx, fy, VestaField(1));
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(e == d + d);
    BOOST_TEST(VestaGroupProjective() == e + f);
    BOOST_TEST(c == PallasGroupProjective() + c);
    BOOST_TEST(c == c + PallasGroupProjective());
    BOOST_TEST(VestaGroupProjective() == VestaGroupProjective() + VestaGroupProjective());
}

BOOST_AUTO_TEST_CASE(groupMulProjective) {
    PallasField ax; std::istringstream("1cb441132f1df394ea0b892518b5f8143814ca5afb8bfcb2cd0b8eaba568b29c") >> ax;
    PallasField ay; std::istringstream("1b01d848ea1769e4e319244446ceebeab80d1687ecd75e1191f8c158a02aaec6") >> ay;
    PallasField cx; std::istringstream("3ae71da7c530d0bbb097cc6b688bb849d1ee146e167637e27486eb874a015ded") >> cx;
    PallasField cy; std::istringstream("101f7a91b0e870b0626c7234eb0024120b66bd06109e55f892fdd00bd5192419") >> cy;
    PallasGroupProjective a(ax, ay, PallasField(1));
    VestaField   b; std::istringstream("27d286de826c7abc89876e85217410148a67ed053968ac6d326ae99eeb11d7f1") >> b;
    PallasGroupProjective c(cx, cy, PallasField(1));
    PallasField  d; std::istringstream("08f41a93bb8c52e757404c04e2519c5f66b126176b9f7307de457606b2be8946") >> d;
    BOOST_TEST(c == a * b);
    BOOST_TEST(a == a * VestaField(1));
    BOOST_TEST(PallasGroupProjective() == a * VestaField(0));
    BOOST_TEST(VestaGroupProjective() == VestaGroupProjective() * d);
}

BOOST_AUTO_TEST_SUITE_END()
