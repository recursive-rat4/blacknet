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

#ifndef BLACKNET_CRYPTO_POSEIDON2LM62_H
#define BLACKNET_CRYPTO_POSEIDON2LM62_H

#include "jive.h"
#include "lm62.h"
#include "poseidon2.h"
#include "sponge.h"

namespace blacknet::crypto {

struct Poseidon2LM62SpongeParams {
    using F = LM62Ring;

    constexpr static const std::size_t a = 3;
    constexpr static const std::size_t t = 12;

    constexpr static const std::size_t rb = 4;
    constexpr static const std::size_t rp = 41;
    constexpr static const std::size_t re = 4;

    constexpr static const std::array<F, t*rb> rcb = std::array<LM62Ring, 48>{
        LM62Ring("03024d1b83f60218"),
        LM62Ring("1f1a7bee0137a8ff"),
        LM62Ring("10ce767b64d4e9ae"),
        LM62Ring("1ba3e925e5275d6f"),
        LM62Ring("0ddb55c9acf4c3d3"),
        LM62Ring("15d2204430f7f056"),
        LM62Ring("12e852024afa2549"),
        LM62Ring("1d0417d5ef70b7b9"),
        LM62Ring("0a897e2d6657d9a4"),
        LM62Ring("1dbb94d1495c3671"),
        LM62Ring("0f868ace3795d3a6"),
        LM62Ring("0fc59d2c6f00f258"),
        LM62Ring("04b79c44a6b55a45"),
        LM62Ring("2699bb2d5d86f988"),
        LM62Ring("0ef8429ae4169291"),
        LM62Ring("080b4724fd36411d"),
        LM62Ring("0521b1bf257fc0cc"),
        LM62Ring("11d20b58a3b0e20b"),
        LM62Ring("17777b58fad9bc8d"),
        LM62Ring("0c8296b0eb611873"),
        LM62Ring("0b6fe12a3017bd02"),
        LM62Ring("27cd07baf1d54612"),
        LM62Ring("03f668b9e95c3d46"),
        LM62Ring("17965f8b267e9b29"),
        LM62Ring("17ec002799a53ecd"),
        LM62Ring("18c2e4da884d7967"),
        LM62Ring("0d2d03a201a8b1bd"),
        LM62Ring("1cf83c13b5dfd566"),
        LM62Ring("02c09aebbac62332"),
        LM62Ring("1e5dff7c99066109"),
        LM62Ring("00cf91b3b2e30a11"),
        LM62Ring("1273d455c17e82d4"),
        LM62Ring("123c53eebb61ea27"),
        LM62Ring("22a40f0954540bc6"),
        LM62Ring("09b4d5ef28b579e2"),
        LM62Ring("1cb88afcb2d6fa8f"),
        LM62Ring("1e20863386446c76"),
        LM62Ring("082f7661b0473090"),
        LM62Ring("05c356de25174843"),
        LM62Ring("174507c5b4300e31"),
        LM62Ring("1f62448269fafcfc"),
        LM62Ring("19f0ba1a3860dd61"),
        LM62Ring("142e33e989697e9b"),
        LM62Ring("26a741336a3448a7"),
        LM62Ring("1ce2d0e613114246"),
        LM62Ring("09e87a70f0ab60d9"),
        LM62Ring("22ea794ae2531f92"),
        LM62Ring("14fafa88499e24a9"),
    };
    constexpr static const std::array<F, rp> rcp = std::array<LM62Ring, 41>{
        LM62Ring("171e82213c62e854"),
        LM62Ring("0aeb5ce7d2158a52"),
        LM62Ring("028e45fd4cbece76"),
        LM62Ring("241edefb413be5b8"),
        LM62Ring("03ee4347467fa135"),
        LM62Ring("11098ecdf47f35dc"),
        LM62Ring("000d97aac5fba569"),
        LM62Ring("11b4cda545e82d9c"),
        LM62Ring("20adab887723b7a1"),
        LM62Ring("229d8a912b6c125c"),
        LM62Ring("0173ffc739e26ec8"),
        LM62Ring("02af311c83601ee3"),
        LM62Ring("27c0658fa74b89e9"),
        LM62Ring("1fdf05bcb12428a1"),
        LM62Ring("1655edddf5d0a00e"),
        LM62Ring("1bf5216a2d6a3fd9"),
        LM62Ring("0bae99e9b4af7922"),
        LM62Ring("04e6ee0fe7290d46"),
        LM62Ring("181a6f9071127e32"),
        LM62Ring("25bfb4520713d407"),
        LM62Ring("0e4c3133823eb7d1"),
        LM62Ring("2360ee9ad31e2e4f"),
        LM62Ring("1e507be23e27fb4c"),
        LM62Ring("1da5505b43911dad"),
        LM62Ring("1ad00ae726566b0a"),
        LM62Ring("04368d8ff7e2d870"),
        LM62Ring("1a57a8464c27b254"),
        LM62Ring("1c68cef6d400e30d"),
        LM62Ring("15a9e541ba069d8a"),
        LM62Ring("0a4cfa8841b5010f"),
        LM62Ring("06ea82581edd03fe"),
        LM62Ring("195df3d8a7167dac"),
        LM62Ring("0e8f446cafa76d0a"),
        LM62Ring("16f6c9a69e7fb4c5"),
        LM62Ring("01745b3fcef84bad"),
        LM62Ring("13cd0cbde4bb1b38"),
        LM62Ring("22dfb341ac1bad46"),
        LM62Ring("13aa9b2dfc05c688"),
        LM62Ring("105f2f26192576dd"),
        LM62Ring("2107d2a0b329d613"),
        LM62Ring("175b52c635b13d54"),
    };
    constexpr static const std::array<F, t*re> rce = std::array<LM62Ring, 48>{
        LM62Ring("0763a89d47be7843"),
        LM62Ring("1c14db8fcb974d20"),
        LM62Ring("0811247b71a2ee88"),
        LM62Ring("1ce771070ab6d2bf"),
        LM62Ring("2734e67d0a54b8f8"),
        LM62Ring("0b1ecf288ea482a7"),
        LM62Ring("1ca2c05587c29792"),
        LM62Ring("1d15d923ad1d1723"),
        LM62Ring("0569c1d679be6271"),
        LM62Ring("2669598094164a7e"),
        LM62Ring("0c00d36a8298b397"),
        LM62Ring("0d1c1375facdbcca"),
        LM62Ring("137436ade742cea4"),
        LM62Ring("1c0726b8db304eb1"),
        LM62Ring("071d07498c245b1f"),
        LM62Ring("24de3d83c476a137"),
        LM62Ring("15a6e517e3e2a18d"),
        LM62Ring("0c80e4c495f2ef9f"),
        LM62Ring("06fde25b91e67a89"),
        LM62Ring("278a38a700357957"),
        LM62Ring("14f86e7c16349090"),
        LM62Ring("0a5fdb156dc90b5e"),
        LM62Ring("0afc99eac9d110a0"),
        LM62Ring("032501668981be4b"),
        LM62Ring("254d5be08b9268d0"),
        LM62Ring("01d005b95981ef6b"),
        LM62Ring("0474ddd5609448d3"),
        LM62Ring("1bb93d4e65f3c707"),
        LM62Ring("17a967e89b58dc92"),
        LM62Ring("24c9682400430422"),
        LM62Ring("227e8651cb964ab8"),
        LM62Ring("14110873a3855ac2"),
        LM62Ring("08086c5a46cbdb58"),
        LM62Ring("148395d131ddd3a0"),
        LM62Ring("09db1b16f6049bc1"),
        LM62Ring("14f990017703468f"),
        LM62Ring("0355ac490398651d"),
        LM62Ring("23983e5da556b31d"),
        LM62Ring("00878d23ff3095e5"),
        LM62Ring("0a6928b31b27f6d2"),
        LM62Ring("15c3ec1b6a96b7be"),
        LM62Ring("0fbf84efddc58be6"),
        LM62Ring("0a06393586a643c7"),
        LM62Ring("21d08a6a83af4ddc"),
        LM62Ring("1f2fd837e70bc112"),
        LM62Ring("21941f70714c5435"),
        LM62Ring("0cbb0366496d1708"),
        LM62Ring("0d714b51f53bddff"),
    };
    constexpr static const std::array<F, t> m = std::array<LM62Ring, 12>{
        LM62Ring("23f2f085543425f2"),
        LM62Ring("0c897630b5a7fd8a"),
        LM62Ring("26a4bc02c2b1bb3c"),
        LM62Ring("108e176f13367f38"),
        LM62Ring("0cccbbb747bfa51b"),
        LM62Ring("1ff239d067c6ab8b"),
        LM62Ring("0830d861a3197852"),
        LM62Ring("20d634f00821025a"),
        LM62Ring("091685f2738285a3"),
        LM62Ring("11f49117a201e4f1"),
        LM62Ring("0cb10d3261180356"),
        LM62Ring("061e73bb02d2887d"),
    };
};

struct Poseidon2LM62JiveParams {
    using F = LM62Ring;

    constexpr static const std::size_t a = 3;
    constexpr static const std::size_t t = 8;

    constexpr static const std::size_t rb = 4;
    constexpr static const std::size_t rp = 40;
    constexpr static const std::size_t re = 4;

    constexpr static const std::array<F, t*rb> rcb = std::array<LM62Ring, 32>{
        LM62Ring("0ff28fe5037895d5"),
        LM62Ring("0632bd1b2736a0bd"),
        LM62Ring("0f9a13edd5616b3e"),
        LM62Ring("13dac5de0c2b559b"),
        LM62Ring("0f8cba31ff2b46bd"),
        LM62Ring("0ac282881d8134fe"),
        LM62Ring("005123fa05634b71"),
        LM62Ring("229bfb0fba48a7a3"),
        LM62Ring("20dc34f577f26aac"),
        LM62Ring("28333e22a16e659d"),
        LM62Ring("1406e2da4c900f7c"),
        LM62Ring("0542d0a76dabcda9"),
        LM62Ring("0b4d47803e11b5d0"),
        LM62Ring("05c6456251b7fc21"),
        LM62Ring("172d4642267cf993"),
        LM62Ring("19c045e3f1ee537a"),
        LM62Ring("14985decdf0602a4"),
        LM62Ring("203960357525d6f3"),
        LM62Ring("16398270dcec2138"),
        LM62Ring("0dbc409701aa4586"),
        LM62Ring("110fdf9524a2daea"),
        LM62Ring("004b366fddd53ba2"),
        LM62Ring("070aadc7b14274f4"),
        LM62Ring("20d575764abfae51"),
        LM62Ring("125778abe47201e8"),
        LM62Ring("0a68c9bee9430087"),
        LM62Ring("1e8c9dab41a4400f"),
        LM62Ring("1ee2892380ef4aac"),
        LM62Ring("0bc34449f155c5d9"),
        LM62Ring("1325c094b2ce0d4c"),
        LM62Ring("23e760d3c7fbeb39"),
        LM62Ring("265a98090b8370fc"),
    };
    constexpr static const std::array<F, rp> rcp = std::array<LM62Ring, 40>{
        LM62Ring("18b6c6fa8bb6a822"),
        LM62Ring("10b4d87d562f426f"),
        LM62Ring("178a421dbd32511e"),
        LM62Ring("078e53ff8a3e9599"),
        LM62Ring("0877693d0d74bbdf"),
        LM62Ring("247b6e2d265aba71"),
        LM62Ring("11327b430900d886"),
        LM62Ring("0e4ed382ed177b1b"),
        LM62Ring("1f02e21e95a6cb52"),
        LM62Ring("1542d65031b7a47e"),
        LM62Ring("13c77f14b0a12b9d"),
        LM62Ring("08e80fa45cc987d5"),
        LM62Ring("185dd19d01bccae4"),
        LM62Ring("0191da4503562c20"),
        LM62Ring("14b433f282df8dad"),
        LM62Ring("012450bb561f7003"),
        LM62Ring("2452167578a4e37f"),
        LM62Ring("12a22fbb017a24dc"),
        LM62Ring("25bc3a8e792399c2"),
        LM62Ring("03a276a997b06aa5"),
        LM62Ring("24ca66e1dedee203"),
        LM62Ring("0902f32363415a04"),
        LM62Ring("0708ecceee220aca"),
        LM62Ring("212238550375ddac"),
        LM62Ring("21d6249958737efa"),
        LM62Ring("26735639546e45f6"),
        LM62Ring("1aed2f252eb9de48"),
        LM62Ring("1fae8a64ae64516a"),
        LM62Ring("05c0bb7f50844eb3"),
        LM62Ring("0201508f0ad83b47"),
        LM62Ring("0dbe32015d3d3424"),
        LM62Ring("20337e2a769f7a11"),
        LM62Ring("11075b69cdbfb295"),
        LM62Ring("04b4ab1f058531d7"),
        LM62Ring("24b0a3b57c372007"),
        LM62Ring("1ae8ccab738bb749"),
        LM62Ring("1b6da7341d9e89b6"),
        LM62Ring("00bc4cbbfbd461ee"),
        LM62Ring("274526b62844580e"),
        LM62Ring("2628664176ac19c5"),
    };
    constexpr static const std::array<F, t*re> rce = std::array<LM62Ring, 32>{
        LM62Ring("15d5042fb7cfdb37"),
        LM62Ring("09a88230cafb7853"),
        LM62Ring("1659f53b0621e0c6"),
        LM62Ring("23d890c16ec43654"),
        LM62Ring("0cf70460f2924f6c"),
        LM62Ring("065dd70d7503b550"),
        LM62Ring("1a37306316de90db"),
        LM62Ring("086752849ebb86d8"),
        LM62Ring("27f320208fb10d81"),
        LM62Ring("22b59afa7a608545"),
        LM62Ring("02c9f4a38424a05a"),
        LM62Ring("1c36226b6831ecf8"),
        LM62Ring("07d65bbc8561e07b"),
        LM62Ring("01fa754b36217194"),
        LM62Ring("077000472fb2dfcc"),
        LM62Ring("09b94a10f5bb6302"),
        LM62Ring("13f1b274e36f53c8"),
        LM62Ring("06373ad773daa078"),
        LM62Ring("18fc166dd899cf98"),
        LM62Ring("22fd278a5fb5d8d1"),
        LM62Ring("04a621efb9c19939"),
        LM62Ring("1a5f3daeb2df1467"),
        LM62Ring("129b454c28db144f"),
        LM62Ring("0929e5c549b44469"),
        LM62Ring("0e544c498a0432d4"),
        LM62Ring("08d00c231327b166"),
        LM62Ring("17443e6c900c28ad"),
        LM62Ring("220dc76c2d0fcd70"),
        LM62Ring("1f308cd4fbdafe75"),
        LM62Ring("2492fd9f1005955d"),
        LM62Ring("140b356037c6c482"),
        LM62Ring("06cc8f5fae680869"),
    };
    constexpr static const std::array<F, t> m = std::array<LM62Ring, 8>{
        LM62Ring("18e96852d1f255a1"),
        LM62Ring("274067aecc9d1893"),
        LM62Ring("11d8c6a65d401dbe"),
        LM62Ring("2217d07b96d34afc"),
        LM62Ring("20f92473c9c25164"),
        LM62Ring("115dc25bf3f997ae"),
        LM62Ring("128961e998776b12"),
        LM62Ring("27c05789571937fa"),
    };
};

template<std::array<LM62Ring, 4> IV>
using Poseidon2LM62Sponge = Sponge<
    LM62Ring,
    8,
    4,
    IV,
    Poseidon2<Poseidon2LM62SpongeParams>,
    SpongeMode::Overwrite
>;

using Poseidon2LM62Jive = Jive<
    LM62Ring,
    4,
    Poseidon2<Poseidon2LM62JiveParams>,
    2
>;

}

#endif
