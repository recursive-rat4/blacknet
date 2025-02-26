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

#ifndef BLACKNET_CRYPTO_POSEIDON2PERVUSHIN_H
#define BLACKNET_CRYPTO_POSEIDON2PERVUSHIN_H

#include "jive.h"
#include "pervushin.h"
#include "poseidon2.h"
#include "sponge.h"

struct Poseidon2PervushinSpongeParams {
    using F = PervushinRing;

    constexpr static const std::size_t a = 17;
    constexpr static const std::size_t t = 12;

    constexpr static const std::size_t rb = 4;
    constexpr static const std::size_t rp = 12;
    constexpr static const std::size_t re = 4;

    constexpr static const std::array<F, t*rb> rcb = std::array<PervushinRing, 48>{
        PervushinRing("1bea68551d3b015e"),
        PervushinRing("0e0b69da4246892f"),
        PervushinRing("06b5d4d6cb619607"),
        PervushinRing("081fc5cb5ef8f48d"),
        PervushinRing("0f21d71bf80ce07f"),
        PervushinRing("150c658aeb227124"),
        PervushinRing("02ee3e792332e8ae"),
        PervushinRing("0eebf1b165764cb6"),
        PervushinRing("115e45e6fb6a45d3"),
        PervushinRing("0c57a09021cb05c9"),
        PervushinRing("10d088963861993f"),
        PervushinRing("07f45d080b68141c"),
        PervushinRing("1822a99f95475dad"),
        PervushinRing("1ab4a2f699eedacc"),
        PervushinRing("066d2206cb6b4c66"),
        PervushinRing("14b73c6cb8748636"),
        PervushinRing("1d4de783d4bfc073"),
        PervushinRing("0585bf9dec7f2e80"),
        PervushinRing("016c1a49f443a3a0"),
        PervushinRing("12755af4eef518e7"),
        PervushinRing("08bcf53c6dd5f3bb"),
        PervushinRing("185b3be9bad5d04a"),
        PervushinRing("03a987ddb8f7faee"),
        PervushinRing("06053640fe81014d"),
        PervushinRing("0e1caae45406582c"),
        PervushinRing("0e9ea46dfbadc5fc"),
        PervushinRing("1a790c8c3d4d9e3c"),
        PervushinRing("0fab33b1612f1d30"),
        PervushinRing("06c1b98e49b19d6d"),
        PervushinRing("1a6e50d0ded2508d"),
        PervushinRing("1761336c02c04916"),
        PervushinRing("0c7085946a46fd93"),
        PervushinRing("0c78b181d5e5439c"),
        PervushinRing("01a2200fa9fd0548"),
        PervushinRing("1e546a16cbc58ca9"),
        PervushinRing("1259cfd49ea390a6"),
        PervushinRing("0e5a10cf4f1af897"),
        PervushinRing("0b49567f5a959dcb"),
        PervushinRing("0190905a76353ace"),
        PervushinRing("19d96459612aceba"),
        PervushinRing("1554e770785449e1"),
        PervushinRing("0f0da3e1991fcf97"),
        PervushinRing("01b6c2c6aa77ddf1"),
        PervushinRing("05dd6897b427fb66"),
        PervushinRing("0fc18bcfbd6a418f"),
        PervushinRing("1f8395f8ff136a32"),
        PervushinRing("062e5dffec9b6de5"),
        PervushinRing("146a74cddc8964f5"),
    };
    constexpr static const std::array<F, rp> rcp = std::array<PervushinRing, 12>{
        PervushinRing("1fb1afcf1de89b25"),
        PervushinRing("079d0c916c68d8f7"),
        PervushinRing("15a913b241a35d0b"),
        PervushinRing("07dfadd0d69e16b0"),
        PervushinRing("1852b50a9797a25e"),
        PervushinRing("0eec7299aa17eeb1"),
        PervushinRing("057f7489ace6eb68"),
        PervushinRing("184ea926c1e48931"),
        PervushinRing("0a6b1fb6886cd5e3"),
        PervushinRing("09c7e2b2aeb59896"),
        PervushinRing("1eccf3cc9bf16107"),
        PervushinRing("01c01aa4dcf13ca2"),
    };
    constexpr static const std::array<F, t*re> rce = std::array<PervushinRing, 48>{
        PervushinRing("0d130cb0c9f4b119"),
        PervushinRing("0a858d7a070a35e8"),
        PervushinRing("1702d16b4f6ff971"),
        PervushinRing("079ce000f1bb981b"),
        PervushinRing("18d431d955ed8cd7"),
        PervushinRing("124c3f24e0eafbda"),
        PervushinRing("1d98c2ecaa2e1c4b"),
        PervushinRing("07069f9ced0bdf71"),
        PervushinRing("0d1803105c1d3b6e"),
        PervushinRing("039d825a73fe9fb9"),
        PervushinRing("1535687903111fa2"),
        PervushinRing("0b32c9c6925e4e85"),
        PervushinRing("140ca45c97ad33ff"),
        PervushinRing("1d6348ceb4790df0"),
        PervushinRing("003a1367a070ddc0"),
        PervushinRing("0a347320b1745926"),
        PervushinRing("023dffb9c064d735"),
        PervushinRing("0cb64be6e50dfec3"),
        PervushinRing("1f7d890cbb6a082b"),
        PervushinRing("0f0e9eaffbf69edc"),
        PervushinRing("022ae764c1d21284"),
        PervushinRing("029acc19950d8a2b"),
        PervushinRing("191e6c68e1bb80a0"),
        PervushinRing("1b64b2ae7d158167"),
        PervushinRing("1222826578ea58be"),
        PervushinRing("1db68e91a53408f4"),
        PervushinRing("021d79b6189f4cc8"),
        PervushinRing("0e7ad0ea76407bcc"),
        PervushinRing("014ec965b6e96ca3"),
        PervushinRing("12a8ae29b344eae4"),
        PervushinRing("0f244cc20e64f0cc"),
        PervushinRing("1f6eb055b6917e8a"),
        PervushinRing("182f55d5a18931ff"),
        PervushinRing("12cf53fdae88c372"),
        PervushinRing("0436ea3fc4884fcc"),
        PervushinRing("0e7df543a3bd4e9e"),
        PervushinRing("0189e162bff1604c"),
        PervushinRing("1f2b6af8fcbeed27"),
        PervushinRing("10ff4896d48d6371"),
        PervushinRing("15966f8d48ddc257"),
        PervushinRing("08d45cc7f7b5667e"),
        PervushinRing("05ef0dad2a956bba"),
        PervushinRing("0d303e520990ed56"),
        PervushinRing("1be6e3c6a3d65ec2"),
        PervushinRing("0edf34dcc273ab78"),
        PervushinRing("1f16d269467d0451"),
        PervushinRing("1d58532bd1a60771"),
        PervushinRing("1aed3e535fd6fed7"),
    };
    constexpr static const std::array<F, t> m = std::array<PervushinRing, 12>{
        PervushinRing("0eee0c4eb310a7a7"),
        PervushinRing("0baf08f61c807618"),
        PervushinRing("0c0aa63ae98dc48a"),
        PervushinRing("01d1b9764d0e0f78"),
        PervushinRing("01e24c73473b8561"),
        PervushinRing("160e8b75670d5b0d"),
        PervushinRing("1b1663047fb85fd5"),
        PervushinRing("0be9a348aa0a7d5a"),
        PervushinRing("03bbf04ccacba145"),
        PervushinRing("1832a1e74a32e5d2"),
        PervushinRing("1ea7d86b0bd3d316"),
        PervushinRing("18db28488c73a35f"),
    };
};

struct Poseidon2PervushinJiveParams {
    using F = PervushinRing;

    constexpr static const std::size_t a = 17;
    constexpr static const std::size_t t = 8;

    constexpr static const std::size_t rb = 4;
    constexpr static const std::size_t rp = 12;
    constexpr static const std::size_t re = 4;

    constexpr static const std::array<F, t*rb> rcb = std::array<PervushinRing, 32>{
        PervushinRing("0cb45a089048f620"),
        PervushinRing("0c10af255db88f5c"),
        PervushinRing("06493f1d491f60d4"),
        PervushinRing("173eae52b9d380d3"),
        PervushinRing("11a5a838544a3c1a"),
        PervushinRing("07404edcd7e89e92"),
        PervushinRing("1d40479608c9b9d6"),
        PervushinRing("06b611851d10b789"),
        PervushinRing("00354a30fedeb9e1"),
        PervushinRing("1967922b30e35d36"),
        PervushinRing("150bb950bbecb278"),
        PervushinRing("1c04cfadc662c4ee"),
        PervushinRing("0ad1ed8e7469efb1"),
        PervushinRing("0c44b22126dd10c5"),
        PervushinRing("187512642b323393"),
        PervushinRing("10360a33d92e36e6"),
        PervushinRing("0172841b8ad2dd36"),
        PervushinRing("07a5d90f058ec967"),
        PervushinRing("01b44df13e151be8"),
        PervushinRing("1ed26d3ae133ed27"),
        PervushinRing("07e05246dfcca449"),
        PervushinRing("074b563fe8279746"),
        PervushinRing("11fbf6fca24fe825"),
        PervushinRing("1c69601b617b69f8"),
        PervushinRing("0b17dc8fd2b1f8da"),
        PervushinRing("002a1e612377e372"),
        PervushinRing("1723f59722bbb64f"),
        PervushinRing("07efa3ecf0233197"),
        PervushinRing("00b311bca34874ce"),
        PervushinRing("176e2ca7054e8e74"),
        PervushinRing("0aa822f86df62c82"),
        PervushinRing("13d1caf98f57d3a6"),
    };
    constexpr static const std::array<F, rp> rcp = std::array<PervushinRing, 12>{
        PervushinRing("155c592e291d9e97"),
        PervushinRing("01750c34cc46a1f0"),
        PervushinRing("0fb211f991a780d5"),
        PervushinRing("06495472df01645d"),
        PervushinRing("1219cfb743292363"),
        PervushinRing("04bb9d7be5e4c0f7"),
        PervushinRing("09f5cf65399c4301"),
        PervushinRing("0b5e39ff47dff9b3"),
        PervushinRing("0d9ee0f26e988c0d"),
        PervushinRing("0f5e7ae6fae36530"),
        PervushinRing("11f72bc1bc54b874"),
        PervushinRing("1613f2be80e12d5f"),
    };
    constexpr static const std::array<F, t*re> rce = std::array<PervushinRing, 32>{
        PervushinRing("0fd0f9f6fcdc7f49"),
        PervushinRing("03149cc9f68b55ad"),
        PervushinRing("0d8aac8351b29ead"),
        PervushinRing("190f94b6dc9e3d6f"),
        PervushinRing("1568b50bc4bdbf23"),
        PervushinRing("16893566cdfd0171"),
        PervushinRing("14155d8230c76e6f"),
        PervushinRing("02846b79954cbc77"),
        PervushinRing("01d7f2dc47d2af30"),
        PervushinRing("0e2b9ddb4881e796"),
        PervushinRing("12ace083096ee2d8"),
        PervushinRing("0c7b101a1a2e52ea"),
        PervushinRing("07712b54233e515b"),
        PervushinRing("1511f62ae46e2dc2"),
        PervushinRing("0c0bb4da075a298f"),
        PervushinRing("002757a9fdbb4c9e"),
        PervushinRing("0f5cc55693d423ab"),
        PervushinRing("15f37ab84b697cbc"),
        PervushinRing("19c283ac67c1499c"),
        PervushinRing("0b25c59f02da3791"),
        PervushinRing("14298fa04d4fe8ed"),
        PervushinRing("1c3b94b708a44cf1"),
        PervushinRing("05fc7b2996e4929e"),
        PervushinRing("1e41e46e28d89b65"),
        PervushinRing("0d46c336718dde36"),
        PervushinRing("13eaed2096d0d455"),
        PervushinRing("01b56caab2dd914e"),
        PervushinRing("0209f5c5b43d31a6"),
        PervushinRing("19bfd8a77cbbaa77"),
        PervushinRing("18563ad7c351001c"),
        PervushinRing("1c7418fc20672a26"),
        PervushinRing("1fadbde450f44a00"),
    };
    constexpr static const std::array<F, t> m = std::array<PervushinRing, 8>{
        PervushinRing("1e1a6271f0929a74"),
        PervushinRing("0da614dc89563cbc"),
        PervushinRing("16d156c6747508bf"),
        PervushinRing("0a00ce5c502d92e5"),
        PervushinRing("16919b4feb3a6563"),
        PervushinRing("1ced093471943ce6"),
        PervushinRing("15d6ce01acb63062"),
        PervushinRing("17355fc9a26a7fed"),
    };
};

template<std::array<PervushinRing, 4> IV>
using Poseidon2PervushinSponge = Sponge<
    PervushinRing,
    8,
    4,
    IV,
    Poseidon2<Poseidon2PervushinSpongeParams>
>;

using Poseidon2PervushinJive = Jive<
    PervushinRing,
    4,
    Poseidon2<Poseidon2PervushinJiveParams>,
    2
>;

#endif
