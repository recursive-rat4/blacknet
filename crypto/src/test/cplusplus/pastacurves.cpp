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

#include "pastacurves.h"

BOOST_AUTO_TEST_SUITE(PastaCurves)

BOOST_AUTO_TEST_CASE(fieldAdd) {
    constexpr const char* const a("2c5a3233336a186012edd7a62943cf0ae38a93b9454d5791b9825d4531fbf11c");
    constexpr const char* const b("34a99c1d1ad68aeb1d35bcf74ddb040b86ba0a05331200ef3e995b42c73be34a");
    constexpr PallasField a1(a);
    constexpr PallasField b1(b);
    constexpr PallasField c1("2103ce504e40a34b3023949d771ed31647fe04c26f125f655eee879af937d465");
    constexpr VestaField  a2(a);
    constexpr VestaField  b2(b);
    constexpr VestaField  c2("2103ce504e40a34b3023949d771ed31647fe04c26ecaafa36bd4cd66f937d465");
    BOOST_TEST(c1 == a1 + b1);
    BOOST_TEST(c2 == a2 + b2);
    BOOST_TEST(c1 == PallasField(0) + c1);
    BOOST_TEST(c2 == c2 + VestaField(0));
    BOOST_TEST(PallasField(1) == PallasField(1) + PallasField(0));
    BOOST_TEST(VestaField(1) == VestaField(0) + VestaField(1));
}

BOOST_AUTO_TEST_CASE(fieldMul) {
    constexpr const char* const a("11640cdb3d3a126dabde403009808a4cae45ec00ffac7480d80ac9142abb607f");
    constexpr const char* const b("0a5111b1ee7f41260df2a030fc99d5aa095ae34332a190ba7ca6d9b54a5d1c85");
    constexpr PallasField a1(a);
    constexpr PallasField b1(b);
    constexpr PallasField c1("0b5842e91b2c5b9b253f653330dcf9d57d1d745479140a959684c13a5a25b6e6");
    constexpr VestaField  a2(a);
    constexpr VestaField  b2(b);
    constexpr VestaField  c2("0158030f7f4f7138ea54d0e0a8797e99ee4c3526ef9c67ccede788174b1f2172");
    BOOST_TEST(c1 == a1 * b1);
    BOOST_TEST(c2 == a2 * b2);
    BOOST_TEST(PallasField(0) == PallasField(0) * c1);
    BOOST_TEST(VestaField(0) == c2 * VestaField(0));
    BOOST_TEST(c1 == c1 * PallasField(1));
    BOOST_TEST(c2 == VestaField(1) * c2);
}

BOOST_AUTO_TEST_CASE(fieldSub) {
    constexpr const char* const a("063c6fa6bc7df187ee00659a73a97b1589892a4ae753fe00c7b3764ddd663cd2");
    constexpr const char* const b("20ac2a42b38f940e1bdc81e7b258588c04aee2f11a782e579033601a00df0730");
    constexpr PallasField a1(a);
    constexpr PallasField b1(b);
    constexpr PallasField c1("2590456408ee5d79d223e3b2c1512289a720e055d628c8c4d0ad4720dc8735a3");
    constexpr PallasField d1("1a6fba9bf711a2862ddc1c4d3eaedd767b25b8a633243056c87fe9cc2378ca5e");
    constexpr VestaField  a2(a);
    constexpr VestaField  b2(b);
    constexpr VestaField  c2("2590456408ee5d79d223e3b2c1512289a720e055d6707886c3c70154dc8735a3");
    constexpr VestaField  d2("1a6fba9bf711a2862ddc1c4d3eaedd767b25b8a633243056c87fe9cc2378ca5e");
    BOOST_TEST(c1 == a1 - b1);
    BOOST_TEST(d1 == b1 - a1);
    BOOST_TEST(c2 == a2 - b2);
    BOOST_TEST(d2 == b2 - a2);
    BOOST_TEST(c1 == c1 - PallasField(0));
    BOOST_TEST(c2 == c2 - VestaField(0));
    BOOST_TEST(PallasField(0) == PallasField(1) - PallasField(1));
    BOOST_TEST(VestaField(0) == VestaField(1) - VestaField(1));
}

BOOST_AUTO_TEST_CASE(fieldDiv) {
    constexpr const char* const a("3faced132f5641f57b1162d06ed827d8ca9fa69f0c7b14822818eef4db6f6fdc");
    constexpr const char* const b("152d43a9a19991aa7f8c98ed185a79eda9b2562e4c456bb554c0c0d4d0362904");
    constexpr PallasField a1(a);
    constexpr PallasField b1(b);
    constexpr PallasField c1("3112d3dbd9cb47dd10c20edd49686b9713d5160fb2560360acc84d06bada7442");
    constexpr VestaField  a2(a);
    constexpr VestaField  b2(b);
    constexpr VestaField  c2("0e1fd01ec64fffe6a6fc237d1608308ddaa1efcb579ea243a347caaf8778061c");
    BOOST_TEST(c1 == a1 / b1);
    BOOST_TEST(c2 == a2 / b2);
    BOOST_TEST(PallasField(0) == PallasField(0) / c1);
    BOOST_CHECK_THROW(c2 / VestaField(0), ArithmeticException);
    BOOST_TEST(PallasField(1) == PallasField(1) / PallasField(1));
    BOOST_TEST(c2 == c2 / VestaField(1));
}

BOOST_AUTO_TEST_CASE(fieldNeg) {
    constexpr const char* const a("12610bc44a0bbc319a91fc24e99a98ef2bd29a2b535bbd1a74bc100a698e34fa");
    constexpr PallasField a1(a);
    constexpr VestaField  a2(a);
    constexpr PallasField b1("2d9ef43bb5f443ce656e03db16656710f673fed0b5f13c01247120e29671cb07");
    constexpr VestaField  b2("2d9ef43bb5f443ce656e03db16656710f673fed0b638ebc3178adb169671cb07");
    BOOST_TEST(b1 == -a1);
    BOOST_TEST(b2 == -a2);
    BOOST_TEST(PallasField(0) == -PallasField(0));
    BOOST_TEST(VestaField(0) == -VestaField(0));
    BOOST_TEST(PallasField(1) == -(-PallasField(1)));
    BOOST_TEST(VestaField(1) == -(-VestaField(1)));
    BOOST_TEST(PallasField(1) == -PallasField(-1));
}

BOOST_AUTO_TEST_CASE(fieldSquare) {
    constexpr PallasField a("2f4564953a3b3bf9fffa19e805dfcd1b1b8381501d83664a5203d7cafa95c2ad");
    constexpr PallasField b("2e4f0f106b3a0c9948816bf44d2587f755014bcbfb7150a2030c0f3eb82402b1");
    BOOST_TEST(b == a.square());
    BOOST_TEST(VestaField(0) == VestaField(0).square());
    BOOST_TEST(VestaField(1) == VestaField(1).square());
}

BOOST_AUTO_TEST_CASE(fieldInv) {
    constexpr PallasField a("0f34fe2fd15703dc7eba4a68d48fa9ee0e9ab8746f759eb8fc23828a4aa48900");
    constexpr PallasField b("087f2909b3c53a656a9f0f126b8458afa89ececeb5676d93c9d4594c4aacc34d");
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(!VestaField(0).invert());
}

BOOST_AUTO_TEST_CASE(fieldSqrt) {
    constexpr const char* const a("35aeb661a5f2e7df341a8f256036c025e07b8e45002f7d9da0c8f7b5aa744aea");
    constexpr const char* const b("39fce7dbf35569b5dc603860e3254bf9e61e3b57ba958a05a121b318906fe126");
    constexpr PallasField a1(a);
    constexpr PallasField b1(b);
    constexpr PallasField c1("344a642baaa8f21985d0757617709370cdc5b87574ecd97b4cf3c9d915689609");
    constexpr VestaField  a2(a);
    constexpr VestaField  b2(b);
    constexpr VestaField  c2("2fd1206ca31cb1de80ffb18d6b4e5095edafca2beb056dfe0125bf1e0cae890a");
    BOOST_TEST(c1 == *a1.sqrt());
    BOOST_TEST(!a2.sqrt());
    BOOST_TEST(c2 == *b2.sqrt());
    BOOST_TEST(!b1.sqrt());
    BOOST_TEST(PallasField(0) == *PallasField(0).sqrt());
    BOOST_TEST(VestaField(0) == *VestaField(0).sqrt());
    BOOST_TEST(PallasField(1) == *PallasField(1).sqrt());
    BOOST_TEST(VestaField(1) == *VestaField(1).sqrt());
}

BOOST_AUTO_TEST_CASE(groupNegAffine) {
    constexpr PallasField ax("2c998f5cd6f89a5323244238dcb0e122f3c48b690d17895d64c622fe7b134872");
    constexpr PallasField ay("2376d983140e67283c34cb1b20d3a6889b55892b51c224c059ba1f97a768959b");
    constexpr PallasField bx("2c998f5cd6f89a5323244238dcb0e122f3c48b690d17895d64c622fe7b134872");
    constexpr PallasField by("1c89267cebf198d7c3cb34e4df2c597786f10fd0b78ad45b3f73115558976a66");
    constexpr VestaField  cx("2b84f575fc91b8f506713c696425fd86ea1f134bdb0f2821816f00ab1eeeaa92");
    constexpr VestaField  cy("179180e8abc3d15ed1d6bc287b7debe66b7c386cad750458ad956514255556bd");
    constexpr VestaField  dx("2b84f575fc91b8f506713c696425fd86ea1f134bdb0f2821816f00ab1eeeaa92");
    constexpr VestaField  dy("286e7f17543c2ea12e2943d784821419b6ca608f5c1fa484deb1860cdaaaa944");
    PallasGroupAffine a(ax, ay);
    PallasGroupAffine b(bx, by);
    VestaGroupAffine  c(cx, cy);
    VestaGroupAffine  d(dx, dy);
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(PallasGroupAffine() == -PallasGroupAffine());
    BOOST_TEST(VestaGroupAffine() == -VestaGroupAffine());
}

BOOST_AUTO_TEST_CASE(groupSubAffine) {
    constexpr PallasField ax("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c");
    constexpr PallasField ay("01a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15");
    constexpr PallasField bx("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c");
    constexpr PallasField by("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba");
    constexpr PallasField cx("3c8ed394b958488903020f14049fde9acb64b089af95809150f2df503eaa8073");
    constexpr PallasField cy("2b7e6a10fecffa0b3b873b40c8cd3df4f7417a5ccc84ca97554fbf0945a8925f");
    constexpr VestaField  dx("124713cd5616381192fdb5bb7868aba8a48952687874b05f8ca79ffeca50fcb6");
    constexpr VestaField  dy("1b550ae837f5c48f1c37c3f0ff55894742917bc8e320ee137012cd563db3ab0a");
    constexpr VestaField  ex("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  ey("01d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0");
    constexpr VestaField  fx("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  fy("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61");
    PallasGroupAffine a(ax, ay);
    PallasGroupAffine b(bx, by);
    PallasGroupAffine c(cx, cy);
    VestaGroupAffine  d(dx, dy);
    VestaGroupAffine  e(ex, ey);
    VestaGroupAffine  f(fx, fy);
    BOOST_TEST(c == a - b);
    BOOST_TEST(c == -b + a);
    BOOST_TEST(VestaGroupAffine() == d - d);
    BOOST_TEST(d == e - f);
    BOOST_TEST(-c == PallasGroupAffine() - c);
    BOOST_TEST(c == c - PallasGroupAffine());
    BOOST_TEST(VestaGroupAffine() == VestaGroupAffine() - VestaGroupAffine());
}

BOOST_AUTO_TEST_CASE(groupAddAffine) {
    constexpr PallasField ax("1e3dbd8ef7121f586a32c8789be6c1bd516ea0b7b5e00d356527f3b9137c7f13");
    constexpr PallasField ay("0c09c8b193a30e6989afa1cd8e3f468529cc2294b5111c80dc53080d10a133e3");
    constexpr PallasField bx("172c422e616dc9017cb392143dcdb133e1071d8e87806ccd9b222d82665aac69");
    constexpr PallasField by("0fb0e51efc9e8cd9c0a70e8fa507ec59fcb5da21d8cac79550c4f98d1dc2d362");
    constexpr PallasField cx("3105fd2e4cf209b0db4e0e0772661ffaee9083b4e5faac71251d9ddbf05c2f23");
    constexpr PallasField cy("067e082d0d17fffdd4de37c218a55e188dbb09200621dad577fab3b592cf9ef4");
    constexpr VestaField  dx("3d3b0ea90d13082aa6862f0dac1e211c286614f222bafe7210862d448ef0e466");
    constexpr VestaField  dy("2b63efb469e111e71293b98fbe5008688cb8de0ca571a0075ea200e74abca6f9");
    constexpr VestaField  ex("1f85aa11a81f4464c19b28e5c55ace5b51689ef25f63156cce7d59e28969a639");
    constexpr VestaField  ey("3816248bb82336b770bc06e56883e8fa92c4557f4b16f1ab9fbd831db7750df8");
    constexpr VestaField  fx("1f85aa11a81f4464c19b28e5c55ace5b51689ef25f63156cce7d59e28969a639");
    constexpr VestaField  fy("07e9db7447dcc9488f43f91a977c17058f82437cbe7db731ec896803488af209");
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
    constexpr PallasField ax("3aed134ed42ad34f18db7529fb0ed4470dbb0a157d676eca74f7789208b87676");
    constexpr PallasField ay("2a7a1566f8a07fe9bc87e23a8556e2e144afbe659053d2bfcbbaaa5a42ed856b");
    constexpr PallasField cx("2a0da0b30d7ff6d2956f3eeb2f72dc75310b85f70aa9123640ed78f1b6c3ff03");
    constexpr PallasField cy("2ddbebbf3c0412bc46ffaec08aaebc3c6bd717f3205bb841814983d016f79ec0");
    constexpr PallasGroupAffine a(ax, ay);
    constexpr VestaField   b("0e18ddb951f8a3a10c33028e6cd15a9b4480c3c825f515b6da24b75e7c813623");
    constexpr PallasGroupAffine c(cx, cy);
    constexpr PallasField  d("251d364ed569cbf14184665ce3fa321e9678002959e04609d1a0ecc692cee9e1");
    BOOST_TEST(c == a * b);
    BOOST_TEST(a == a * VestaField(1));
    BOOST_TEST(PallasGroupAffine() == a * VestaField(0));
    BOOST_TEST(VestaGroupAffine() == VestaGroupAffine() * d);
}

BOOST_AUTO_TEST_CASE(groupNegJacobian) {
    constexpr PallasField ax("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5");
    constexpr PallasField ay("2f89c29d9ae36f7c0f20ef5d73f85cea5fdc1cfeae3b96e36c377d3b2f1afb4d");
    constexpr PallasField bx("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5");
    constexpr PallasField by("10763d62651c9083f0df10a28c07a315c26a7bfd5b1162382cf5b3b1d0e504b4");
    constexpr VestaField  cx("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17");
    constexpr VestaField  cy("0e2e3683b3e12f2b986560a0b3a208f29066185aad807056b440e687f990a70a");
    constexpr VestaField  dx("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17");
    constexpr VestaField  dy("31d1c97c4c1ed0d4679a9f5f4c5df70d91e080a15c143886d8060499066f58f7");
    PallasGroupJacobian a(ax, ay, PallasField(1));
    PallasGroupJacobian b(bx, by, PallasField(1));
    VestaGroupJacobian  c(cx, cy, VestaField(1));
    VestaGroupJacobian  d(dx, dy, VestaField(1));
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(PallasGroupJacobian() == -PallasGroupJacobian());
    BOOST_TEST(VestaGroupJacobian() == -VestaGroupJacobian());
}

BOOST_AUTO_TEST_CASE(groupSubJacobian) {
    constexpr PallasField ax("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c");
    constexpr PallasField ay("01a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15");
    constexpr PallasField bx("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c");
    constexpr PallasField by("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba");
    constexpr PallasField cx("3c8ed394b958488903020f14049fde9acb64b089af95809150f2df503eaa8073");
    constexpr PallasField cy("2b7e6a10fecffa0b3b873b40c8cd3df4f7417a5ccc84ca97554fbf0945a8925f");
    constexpr VestaField  dx("124713cd5616381192fdb5bb7868aba8a48952687874b05f8ca79ffeca50fcb6");
    constexpr VestaField  dy("1b550ae837f5c48f1c37c3f0ff55894742917bc8e320ee137012cd563db3ab0a");
    constexpr VestaField  ex("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  ey("01d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0");
    constexpr VestaField  fx("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  fy("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61");
    PallasGroupJacobian a(ax, ay, PallasField(1));
    PallasGroupJacobian b(bx, by, PallasField(1));
    PallasGroupJacobian c(cx, cy, PallasField(1));
    VestaGroupJacobian  d(dx, dy, VestaField(1));
    VestaGroupJacobian  e(ex, ey, VestaField(1));
    VestaGroupJacobian  f(fx, fy, VestaField(1));
    BOOST_TEST(c == a - b);
    BOOST_TEST(c == -b + a);
    BOOST_TEST(VestaGroupJacobian() == d - d);
    BOOST_TEST(d == e - f);
    BOOST_TEST(-c == PallasGroupJacobian() - c);
    BOOST_TEST(c == c - PallasGroupJacobian());
    BOOST_TEST(VestaGroupJacobian() == VestaGroupJacobian() - VestaGroupJacobian());
}

BOOST_AUTO_TEST_CASE(groupAddJacobian) {
    constexpr PallasField ax("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c");
    constexpr PallasField ay("01a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15");
    constexpr PallasField bx("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c");
    constexpr PallasField by("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba");
    constexpr PallasField cx("0201da427944269dee8b83e3cb8400f980a26ca9b89e6787e97c70ab09460d2e");
    constexpr PallasField cy("1d7929dcd5888af7651396fbcf1c145e178f5cbbbc9f497496c9b531692df787");
    constexpr VestaField  dx("2e3f99264efffdf2e6a620de2fd553baadc50da215ba7d2cace02a1843cab60e");
    constexpr VestaField  dy("3076516f0a8d132db8e5d71e15f1455c39b6cffa67946cd15b5daeb331557ba4");
    constexpr VestaField  ex("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  ey("01d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0");
    constexpr VestaField  fx("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  fy("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61");
    PallasGroupJacobian a(ax, ay, PallasField(1));
    PallasGroupJacobian b(bx, by, PallasField(1));
    PallasGroupJacobian c(cx, cy, PallasField(1));
    VestaGroupJacobian  d(dx, dy, VestaField(1));
    VestaGroupJacobian  e(ex, ey, VestaField(1));
    VestaGroupJacobian  f(fx, fy, VestaField(1));
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(e == d + d);
    BOOST_TEST(VestaGroupJacobian() == e + f);
    BOOST_TEST(c == PallasGroupJacobian() + c);
    BOOST_TEST(c == c + PallasGroupJacobian());
    BOOST_TEST(VestaGroupJacobian() == VestaGroupJacobian() + VestaGroupJacobian());
}

BOOST_AUTO_TEST_CASE(groupMulJacobian) {
    constexpr PallasField ax("1cb441132f1df394ea0b892518b5f8143814ca5afb8bfcb2cd0b8eaba568b29c");
    constexpr PallasField ay("1b01d848ea1769e4e319244446ceebeab80d1687ecd75e1191f8c158a02aaec6");
    constexpr PallasField cx("3ae71da7c530d0bbb097cc6b688bb849d1ee146e167637e27486eb874a015ded");
    constexpr PallasField cy("101f7a91b0e870b0626c7234eb0024120b66bd06109e55f892fdd00bd5192419");
    constexpr PallasGroupJacobian a(ax, ay, PallasField(1));
    constexpr VestaField   b("27d286de826c7abc89876e85217410148a67ed053968ac6d326ae99eeb11d7f1");
    constexpr PallasGroupJacobian c(cx, cy, PallasField(1));
    constexpr PallasField  d("08f41a93bb8c52e757404c04e2519c5f66b126176b9f7307de457606b2be8946");
    BOOST_TEST(c == a * b);
    BOOST_TEST(a == a * VestaField(1));
    BOOST_TEST(PallasGroupJacobian() == a * VestaField(0));
    BOOST_TEST(VestaGroupJacobian() == VestaGroupJacobian() * d);
}

BOOST_AUTO_TEST_CASE(groupNegProjective) {
    constexpr PallasField ax("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5");
    constexpr PallasField ay("2f89c29d9ae36f7c0f20ef5d73f85cea5fdc1cfeae3b96e36c377d3b2f1afb4d");
    constexpr PallasField bx("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5");
    constexpr PallasField by("10763d62651c9083f0df10a28c07a315c26a7bfd5b1162382cf5b3b1d0e504b4");
    constexpr VestaField  cx("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17");
    constexpr VestaField  cy("0e2e3683b3e12f2b986560a0b3a208f29066185aad807056b440e687f990a70a");
    constexpr VestaField  dx("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17");
    constexpr VestaField  dy("31d1c97c4c1ed0d4679a9f5f4c5df70d91e080a15c143886d8060499066f58f7");
    PallasGroupProjective a(ax, ay, PallasField(1));
    PallasGroupProjective b(bx, by, PallasField(1));
    VestaGroupProjective  c(cx, cy, VestaField(1));
    VestaGroupProjective  d(dx, dy, VestaField(1));
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(PallasGroupProjective() == -PallasGroupProjective());
    BOOST_TEST(VestaGroupProjective() == -VestaGroupProjective());
}

BOOST_AUTO_TEST_CASE(groupSubProjective) {
    constexpr PallasField ax("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c");
    constexpr PallasField ay("01a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15");
    constexpr PallasField bx("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c");
    constexpr PallasField by("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba");
    constexpr PallasField cx("3c8ed394b958488903020f14049fde9acb64b089af95809150f2df503eaa8073");
    constexpr PallasField cy("2b7e6a10fecffa0b3b873b40c8cd3df4f7417a5ccc84ca97554fbf0945a8925f");
    constexpr VestaField  dx("124713cd5616381192fdb5bb7868aba8a48952687874b05f8ca79ffeca50fcb6");
    constexpr VestaField  dy("1b550ae837f5c48f1c37c3f0ff55894742917bc8e320ee137012cd563db3ab0a");
    constexpr VestaField  ex("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  ey("01d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0");
    constexpr VestaField  fx("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  fy("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61");
    PallasGroupProjective a(ax, ay, PallasField(1));
    PallasGroupProjective b(bx, by, PallasField(1));
    PallasGroupProjective c(cx, cy, PallasField(1));
    VestaGroupProjective  d(dx, dy, VestaField(1));
    VestaGroupProjective  e(ex, ey, VestaField(1));
    VestaGroupProjective  f(fx, fy, VestaField(1));
    BOOST_TEST(c == a - b);
    BOOST_TEST(c == -b + a);
    BOOST_TEST(VestaGroupProjective() == d - d);
    BOOST_TEST(d == e - f);
    BOOST_TEST(-c == PallasGroupProjective() - c);
    BOOST_TEST(c == c - PallasGroupProjective());
    BOOST_TEST(VestaGroupProjective() == VestaGroupProjective() - VestaGroupProjective());
}

BOOST_AUTO_TEST_CASE(groupAddProjective) {
    constexpr PallasField ax("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c");
    constexpr PallasField ay("01a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15");
    constexpr PallasField bx("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c");
    constexpr PallasField by("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba");
    constexpr PallasField cx("0201da427944269dee8b83e3cb8400f980a26ca9b89e6787e97c70ab09460d2e");
    constexpr PallasField cy("1d7929dcd5888af7651396fbcf1c145e178f5cbbbc9f497496c9b531692df787");
    constexpr VestaField  dx("2e3f99264efffdf2e6a620de2fd553baadc50da215ba7d2cace02a1843cab60e");
    constexpr VestaField  dy("3076516f0a8d132db8e5d71e15f1455c39b6cffa67946cd15b5daeb331557ba4");
    constexpr VestaField  ex("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  ey("01d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0");
    constexpr VestaField  fx("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8");
    constexpr VestaField  fy("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61");
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
    constexpr PallasField ax("1cb441132f1df394ea0b892518b5f8143814ca5afb8bfcb2cd0b8eaba568b29c");
    constexpr PallasField ay("1b01d848ea1769e4e319244446ceebeab80d1687ecd75e1191f8c158a02aaec6");
    constexpr PallasField cx("3ae71da7c530d0bbb097cc6b688bb849d1ee146e167637e27486eb874a015ded");
    constexpr PallasField cy("101f7a91b0e870b0626c7234eb0024120b66bd06109e55f892fdd00bd5192419");
    constexpr PallasGroupProjective a(ax, ay, PallasField(1));
    constexpr VestaField   b("27d286de826c7abc89876e85217410148a67ed053968ac6d326ae99eeb11d7f1");
    constexpr PallasGroupProjective c(cx, cy, PallasField(1));
    constexpr PallasField  d("08f41a93bb8c52e757404c04e2519c5f66b126176b9f7307de457606b2be8946");
    BOOST_TEST(c == a * b);
    BOOST_TEST(a == a * VestaField(1));
    BOOST_TEST(PallasGroupProjective() == a * VestaField(0));
    BOOST_TEST(VestaGroupProjective() == VestaGroupProjective() * d);
}

BOOST_AUTO_TEST_SUITE_END()
