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

#ifndef BLACKNET_CRYPTO_POSEIDON2PERVUSHIN_H
#define BLACKNET_CRYPTO_POSEIDON2PERVUSHIN_H

#include "pervushin.h"
#include "poseidon2.h"
#include "sponge.h"

struct Poseidon2PervushinParams {
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

using Poseidon2Pervushin = Sponge<
    PervushinRing,
    8,
    4,
    Poseidon2<Poseidon2PervushinParams>
>;

#endif
