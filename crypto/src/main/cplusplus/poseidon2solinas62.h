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

#ifndef BLACKNET_CRYPTO_POSEIDON2SOLINAS62_H
#define BLACKNET_CRYPTO_POSEIDON2SOLINAS62_H

#include "poseidon2.h"
#include "solinas62.h"
#include "sponge.h"

struct Poseidon2Solinas62Params {
    using F = Solinas62Ring;

    constexpr static const std::size_t a = 3;
    constexpr static const std::size_t t = 12;

    constexpr static const std::size_t rb = 4;
    constexpr static const std::size_t rp = 41;
    constexpr static const std::size_t re = 4;

    constexpr static const std::array<F, t*rb> rcb = std::array<Solinas62Ring, 48>{
        Solinas62Ring("03024d1b83f60218"),
        Solinas62Ring("1f1a7bee0137a8ff"),
        Solinas62Ring("10ce767b64d4e9ae"),
        Solinas62Ring("3a48d9c40ac51427"),
        Solinas62Ring("1ba3e925e5275d6f"),
        Solinas62Ring("38dced681d36bcc0"),
        Solinas62Ring("396a250883366677"),
        Solinas62Ring("366ffcf7b739d59d"),
        Solinas62Ring("0ddb55c9acf4c3d3"),
        Solinas62Ring("15d2204430f7f056"),
        Solinas62Ring("12e852024afa2549"),
        Solinas62Ring("1d0417d5ef70b7b9"),
        Solinas62Ring("0a897e2d6657d9a4"),
        Solinas62Ring("1dbb94d1495c3671"),
        Solinas62Ring("370ddccaaee62512"),
        Solinas62Ring("0f868ace3795d3a6"),
        Solinas62Ring("0fc59d2c6f00f258"),
        Solinas62Ring("302e3cf22b9b7b0a"),
        Solinas62Ring("04b79c44a6b55a45"),
        Solinas62Ring("2699bb2d5d86f988"),
        Solinas62Ring("3b87e2e4bf527d17"),
        Solinas62Ring("33cf3e735a2eac48"),
        Solinas62Ring("0ef8429ae4169291"),
        Solinas62Ring("080b4724fd36411d"),
        Solinas62Ring("0521b1bf257fc0cc"),
        Solinas62Ring("11d20b58a3b0e20b"),
        Solinas62Ring("17777b58fad9bc8d"),
        Solinas62Ring("0c8296b0eb611873"),
        Solinas62Ring("0b6fe12a3017bd02"),
        Solinas62Ring("27cd07baf1d54612"),
        Solinas62Ring("03f668b9e95c3d46"),
        Solinas62Ring("17965f8b267e9b29"),
        Solinas62Ring("17ec002799a53ecd"),
        Solinas62Ring("18c2e4da884d7967"),
        Solinas62Ring("33aed1d07d12fe5a"),
        Solinas62Ring("0d2d03a201a8b1bd"),
        Solinas62Ring("1cf83c13b5dfd566"),
        Solinas62Ring("02c09aebbac62332"),
        Solinas62Ring("1e5dff7c99066109"),
        Solinas62Ring("00cf91b3b2e30a11"),
        Solinas62Ring("1273d455c17e82d4"),
        Solinas62Ring("123c53eebb61ea27"),
        Solinas62Ring("22a40f0954540bc6"),
        Solinas62Ring("392d0cfedeb3aa0d"),
        Solinas62Ring("3ab162c3f20d4259"),
        Solinas62Ring("2fd5809cc0c625db"),
        Solinas62Ring("09b4d5ef28b579e2"),
        Solinas62Ring("1cb88afcb2d6fa8f"),
    };
    constexpr static const std::array<F, rp> rcp = std::array<Solinas62Ring, 41>{
        Solinas62Ring("1e20863386446c76"),
        Solinas62Ring("082f7661b0473090"),
        Solinas62Ring("05c356de25174843"),
        Solinas62Ring("174507c5b4300e31"),
        Solinas62Ring("1f62448269fafcfc"),
        Solinas62Ring("19f0ba1a3860dd61"),
        Solinas62Ring("375390906e2f19be"),
        Solinas62Ring("142e33e989697e9b"),
        Solinas62Ring("26a741336a3448a7"),
        Solinas62Ring("3150b771b1644786"),
        Solinas62Ring("1ce2d0e613114246"),
        Solinas62Ring("3bd20b79c6400cf6"),
        Solinas62Ring("09e87a70f0ab60d9"),
        Solinas62Ring("22ea794ae2531f92"),
        Solinas62Ring("14fafa88499e24a9"),
        Solinas62Ring("2ff741641883cbce"),
        Solinas62Ring("171e82213c62e854"),
        Solinas62Ring("0aeb5ce7d2158a52"),
        Solinas62Ring("028e45fd4cbece76"),
        Solinas62Ring("241edefb413be5b8"),
        Solinas62Ring("03ee4347467fa135"),
        Solinas62Ring("2ec493e57cb39507"),
        Solinas62Ring("3bb9f9c5c658cbc7"),
        Solinas62Ring("2e845f2ac69c76f8"),
        Solinas62Ring("321106d15751ef57"),
        Solinas62Ring("3763bf5f0e429c09"),
        Solinas62Ring("2eafa77ba6602716"),
        Solinas62Ring("11098ecdf47f35dc"),
        Solinas62Ring("000d97aac5fba569"),
        Solinas62Ring("11b4cda545e82d9c"),
        Solinas62Ring("33d4044679043229"),
        Solinas62Ring("20adab887723b7a1"),
        Solinas62Ring("229d8a912b6c125c"),
        Solinas62Ring("0173ffc739e26ec8"),
        Solinas62Ring("02af311c83601ee3"),
        Solinas62Ring("2ff83b4700bc99c3"),
        Solinas62Ring("3d5a33e09ccb517d"),
        Solinas62Ring("27c0658fa74b89e9"),
        Solinas62Ring("1fdf05bcb12428a1"),
        Solinas62Ring("1655edddf5d0a00e"),
        Solinas62Ring("33a421d0ec278c81"),
    };
    constexpr static const std::array<F, t*re> rce = std::array<Solinas62Ring, 48>{
        Solinas62Ring("1bf5216a2d6a3fd9"),
        Solinas62Ring("0bae99e9b4af7922"),
        Solinas62Ring("04e6ee0fe7290d46"),
        Solinas62Ring("3646fc4b65923a34"),
        Solinas62Ring("181a6f9071127e32"),
        Solinas62Ring("25bfb4520713d407"),
        Solinas62Ring("0e4c3133823eb7d1"),
        Solinas62Ring("2f8da820c97d79ad"),
        Solinas62Ring("2360ee9ad31e2e4f"),
        Solinas62Ring("36859fe2ee6b19c9"),
        Solinas62Ring("1e507be23e27fb4c"),
        Solinas62Ring("1da5505b43911dad"),
        Solinas62Ring("2c03f61834b60391"),
        Solinas62Ring("1ad00ae726566b0a"),
        Solinas62Ring("04368d8ff7e2d870"),
        Solinas62Ring("3f3f7465cebf4043"),
        Solinas62Ring("2e4a741574d263bb"),
        Solinas62Ring("1a57a8464c27b254"),
        Solinas62Ring("31c4b0d8038714f9"),
        Solinas62Ring("1c68cef6d400e30d"),
        Solinas62Ring("3c4f011697c2581e"),
        Solinas62Ring("15a9e541ba069d8a"),
        Solinas62Ring("0a4cfa8841b5010f"),
        Solinas62Ring("350bc4e042b57394"),
        Solinas62Ring("06ea82581edd03fe"),
        Solinas62Ring("195df3d8a7167dac"),
        Solinas62Ring("0e8f446cafa76d0a"),
        Solinas62Ring("16f6c9a69e7fb4c5"),
        Solinas62Ring("2b82585c1dc9a299"),
        Solinas62Ring("2bde74fcdd260f54"),
        Solinas62Ring("01745b3fcef84bad"),
        Solinas62Ring("13cd0cbde4bb1b38"),
        Solinas62Ring("22dfb341ac1bad46"),
        Solinas62Ring("3ed2ee9f9c356725"),
        Solinas62Ring("2fcb410e8204c731"),
        Solinas62Ring("33c220def0115fd7"),
        Solinas62Ring("2ceaa7ac7785cf32"),
        Solinas62Ring("36daae6053bfcd35"),
        Solinas62Ring("13aa9b2dfc05c688"),
        Solinas62Ring("3eb97263b83e251b"),
        Solinas62Ring("327243ff3bba82f1"),
        Solinas62Ring("389366c48a0d55ed"),
        Solinas62Ring("3eea6a565fa757af"),
        Solinas62Ring("3226d2dc4402d985"),
        Solinas62Ring("105f2f26192576dd"),
        Solinas62Ring("34e52b800a8ce042"),
        Solinas62Ring("2107d2a0b329d613"),
        Solinas62Ring("175b52c635b13d54"),
    };
    constexpr static const std::array<F, t> m = std::array<Solinas62Ring, 12>{
        Solinas62Ring("3d67323c63da8c4b"),
        Solinas62Ring("2947213c8404dda8"),
        Solinas62Ring("3c3cf080900a6d34"),
        Solinas62Ring("0763a89d47be7842"),
        Solinas62Ring("1c14db8fcb974d1f"),
        Solinas62Ring("0811247b71a2ee87"),
        Solinas62Ring("1ce771070ab6d2be"),
        Solinas62Ring("2734e67d0a54b8f7"),
        Solinas62Ring("0b1ecf288ea482a6"),
        Solinas62Ring("1ca2c05587c29791"),
        Solinas62Ring("1d15d923ad1d1722"),
        Solinas62Ring("2e34079861bf1802"),
    };
};

using Poseidon2Solinas62 = Sponge<
    Solinas62Ring,
    8,
    4,
    Poseidon2<Poseidon2Solinas62Params>
>;

#endif
