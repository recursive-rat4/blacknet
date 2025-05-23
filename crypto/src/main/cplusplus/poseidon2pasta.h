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

#ifndef BLACKNET_CRYPTO_POSEIDON2PASTA_H
#define BLACKNET_CRYPTO_POSEIDON2PASTA_H

#include "pastacurves.h"
#include "poseidon2.h"
#include "sponge.h"

namespace blacknet::crypto {

struct Poseidon2PallasSpongeParams {
    using F = PallasField;

    constexpr static const std::size_t a = 5;
    constexpr static const std::size_t t = 3;

    constexpr static const std::size_t rb = 4;
    constexpr static const std::size_t rp = 56;
    constexpr static const std::size_t re = 4;

    constexpr static const std::array<F, t*rb> rcb = std::array<PallasField, 12>{
        PallasField("360d7470611e473d353f628f76d110f34e71162f31003b7057538c2596426303"),
        PallasField("2bab94d7ae222d135dc3c6c5febfaa314908ac2f12ebe06fbdb74213bf63188b"),
        PallasField("150c93fef652fb1c2bf03e1a29aa871fef77e7d736766c5d0939d92753cc5dc8"),
        PallasField("3270661e68928b3a955d55db56dc57c103cc0a60141e894e14259dce537782b2"),
        PallasField("073f116f04122e25a0b7afe4e2057299b407c370f2b5a1ccce9fb9ffc345afb3"),
        PallasField("2a32ec5c4ee5b1837affd09c1f53f5fd55c9cd2061ae93ca8ebad76fc71554d8"),
        PallasField("270326ee039df19e651e2cfc740628ca634d24fc6e2559f22d8ccbe292efeead"),
        PallasField("27c6642ac633bc66dc100fe7fcfa54918af895bce012f182a068fc37c182e274"),
        PallasField("1bdfd8b01401c70ad27f57396989129d710e1fb6ab976a459ca18682e26d7ff9"),
        PallasField("162a14c62f9a89b814b9d6a9c84dd678f4f6fb3f9054d373c832d824261a35ea"),
        PallasField("2d193e0f76de586b2af6f79e3127feeaac0a1fc71e2cf0c0f79824667b5b6bec"),
        PallasField("044ca3cc4a85d73b81696ef1104e674f4feff82984990ff85d0bf58dc8a4aa94"),
    };
    constexpr static const std::array<F, rp> rcp = std::array<PallasField, 56>{
        PallasField("1cbaf2b371dac6a81d0453416d3e235cb8d9e2d4f314f46f6198785f0cd6b9af"),
        PallasField("1d5b2777692c205b0e6c49d061b6b5f4293c4ab038fdbbdc343e07610f3fede5"),
        PallasField("2e9bdbba3dd34bffaa30535bdd749a7e06a9adb0c1e6f962f60e971b8d73b04f"),
        PallasField("2de11886b18011ca8bd5bae36969299fde40fbe26d047b05035a13661f22418b"),
        PallasField("2e07de1780b8a70d0d5b4a3f1841dcd82ab9395c449be947bc998884ba96a721"),
        PallasField("0f69f1854d20ca0cbbdb63dbd52dad16250440a99d6b8af3825e4c2bb74925ca"),
        PallasField("2eb1b25417fe17670d135dc639fb09a46ce5113507f96de9816c059422dc705e"),
        PallasField("115cd0a0643cfb988c24cb44c3fab48aff36c661d26cc42db8b1bdf4953bd82c"),
        PallasField("26ca293f7b2c462d066d7378b999868bbb57ddf14e0f958ade801612311d04cd"),
        PallasField("17bf1b93c4c7e01a2a830aa162412cd90f160bf9f71e967ff5209d14b24820ca"),
        PallasField("35b41a7ac4f3c571a24f8456369c85dfe03c0354bd8cfd3805c86f2e7dc293c5"),
        PallasField("3b1480080523c439435927994849bea964e14d3beb2dddde72ac156af435d09e"),
        PallasField("2cc6810031dc1b0d4950856dc907d57508e286442a2d3eb2271618d874b14c6d"),
        PallasField("25bdbbeda1bde8c1059618e2afd2ef999e517aa93b78341d91f318c09f0cb566"),
        PallasField("392a4a8758e06ee8b95f33c25dde8ac02a5ed0a27b61926cc6313487073f7f7b"),
        PallasField("272a55878a08442b9aa6111f4de009485e6a6fd15db89365e7bbcef02eb5866c"),
        PallasField("2d5b308b0cf02cdfefa13c4e60e26239a6ebba011694dd129b925b3c5b21e0e2"),
        PallasField("16549fc6af2f3b72dd5d293d72e2e5f244dff42f18b46c56ef38c57c311673ac"),
        PallasField("1b10bb7a82afce39fa69c3a2ad52f76d76398265344203119b7126d9b46860df"),
        PallasField("0f1e7505ebd91d2fc79c2df7dc98a3bed1b36968ba0405c090d27f6a00b7dfc8"),
        PallasField("2f313faf0d3f6187537a7497a3b43f46797fd6e3f18eb1caff457756b819bb20"),
        PallasField("3a5cbb6de450b481fa3ca61c0ed15bc55cad11ebf0f7ceb8f0bc3e732ecb26f6"),
        PallasField("3dab54bc9bef688dd92086e253b439d651baa6e20f892b62865527cbca915982"),
        PallasField("06dbfb42b979884de280d31670123f744c24b33b410fefd4368045acf2b71ae3"),
        PallasField("068d6b4608aae810c6f039ea1973a63eb8d2de72e3d2c9eca7fc32d22f18b9d3"),
        PallasField("366ebfafa3ad381c0ee258c9b8fdfccdb868a7d7e1f1f69a2b5dfcc5572555df"),
        PallasField("39678f65512f1ee404db3024f41d3f567ef66d89d044d022e6bc229e95bc76b1"),
        PallasField("21668f016a8063c0d58b7750a3bc2fe1cf82c25f99dc01a4e534c88fe53d85fe"),
        PallasField("39d00994a8a5046a1bc749363e98a768e34dea56439fe1954bef429bc5331608"),
        PallasField("1f9dbdc3f84312636b203bbe12fb3425b163d41605d39f99770c956f60d881b3"),
        PallasField("027745a9cddfad95e5f17b9e0ee0cab6be0bc829fe5e66c69794a9f7c336eab2"),
        PallasField("1cec0803c504b635788d695c61e932122fa43fe20a45c78d52025657abd8aee0"),
        PallasField("123523d75e9fabc172077448ef87cc6eed5082c8dbf31365d3872a9559a03a73"),
        PallasField("1723d1452c9cf02df419b848e5d694bf27feba35975ee7e5001779e3a1d357f4"),
        PallasField("1739d180a16010bdfcc0573d7e61369421c3f776f572836d9dab1ee4dcf96622"),
        PallasField("2d4e6354da9cc554acce32391794b627fafa96fbeb0ab89370290452042d048d"),
        PallasField("153ee6142e535e334a869553c9d007f88f3bd43f99260621670bcf6f8b485dcd"),
        PallasField("0c45bfd3a69aaa65635ef7e7a430b486968ad4424af83700d258d2e2b7782172"),
        PallasField("0adfd53b256a6957f2d56aec831446006897ac0a8ffa5ff10e5633d251f73307"),
        PallasField("315d2ac8ebdbac3c8cd1726b7cbab8ee3f87b28f1c1be4bdac9d36a8b7516d63"),
        PallasField("1b8472712d02eef4cfaec23d2b16883fc9bb60d1f6959879299ce44ea423d8e1"),
        PallasField("3c1cd07efda6ff24bd0b70fa2255eb6f367d2c54e36928c9c4a5404198adf70c"),
        PallasField("136052d26bb3d373687f4e51b2e1dcd34a16073f738f7e0cbbe523aef9ab107a"),
        PallasField("16c96beef6a0a848c1bdd859a1232a1d7b3cfbb873032681676c36c24ef967dd"),
        PallasField("284b38c57ff65c262ab7fed8f499a9fb012387bab4f1662d067eec7f2d6340c4"),
        PallasField("0c5993d175e81f6639e242198897d17cfc06772c1c0411a6af1dff204c922f86"),
        PallasField("03bf7a3f7bd043dafcda655d1ba9c8f9f24887ad48e17759bbf53f67b1f87b15"),
        PallasField("3188fe4ee9f9fafbb0cf999567f00e734c8f9cbe69f0e8279b5cd09e36d8be62"),
        PallasField("171f528ccf6584375a39768c480d61e13af5bf77c1c42652afea99a2ec6c595a"),
        PallasField("12f4175c4ab45afc196e41859b35ef88812c3286ee7000675a0563b9b8e9f1d5"),
        PallasField("3a509e155cb7ebfd8f8fdcf800a9ac697e23e1aabe96cfab0e74d4d369118b79"),
        PallasField("10f2a685df4a27c81a89920e2504c3b3984bc8f2e4c1b69e98712c65678cfd30"),
        PallasField("09e5f49790c8a0e21d8d93d54ab91a0e54573c9333c56321e8a16728cc9d4918"),
        PallasField("352d69bed80ee3e52bf35705d9f84a3442d17ed6ee0fab7e609a740347cf5fea"),
        PallasField("058ee73ba9f3f293491562faf2b190d3c634debd281b76a63a758af6fa84e0e8"),
        PallasField("232f99cc911eddd9cd0f1fc55b1a3250092cb92119bc76be621a132510a43904"),
    };
    constexpr static const std::array<F, t*re> rce = std::array<PallasField, 12>{
        PallasField("201beed7b8f3ab8186c22c6c5d4869f0f9efd52ca6bc2961c3b97c1e301bc213"),
        PallasField("1376dce6580030c6a1c9291d58602f5129388842744a1210bf6b3431ba94e9bc"),
        PallasField("1793199e6fd6ba342b3356c38238f761072ba8b02d92e7226454843c5486d7b3"),
        PallasField("22de7a7488dcc7359fee9c20c87a67df3c66160dc62aacac06a3f1d3b433311b"),
        PallasField("3514d5e9066bb160df8ff37fe2d8edf8dbe0b77fae77e1d030d6e3fd516b47a8"),
        PallasField("30cd3006931ad636f919a00dabbf5fa5ff453d6f900f144a19377427137a81c7"),
        PallasField("253d1a5c5293412741f81a5cf613c8df8f9e4b2cae2ebb515b6a74220692b506"),
        PallasField("035b461c02d79d19a35e9613e7f5fe92851b3a59c990fafc73f666cb86a48e8e"),
        PallasField("23a9928079d175bd5bc00eedd56b93e092b1283c2d5fccde7cfbf86a3aa04780"),
        PallasField("13a7785ae134ea92f1594a0763c611abb5e2ea3436eef957f1e4ccd73fa00a82"),
        PallasField("39fce308b7d43c574962ae3c0da17e313889c57863446d88bbf04f5252de4279"),
        PallasField("1aae18833f8e1d3ac0fdf01662f60d22bef00a08c6ed38d23b57e34489b53fad"),
    };
    constexpr static const std::array<F, t> m = std::array<PallasField, 3>{
        1,
        1,
        2,
    };
};

template<std::array<PallasField, 1> IV>
using Poseidon2PallasSponge = Sponge<
    PallasField,
    2,
    1,
    IV,
    Poseidon2<Poseidon2PallasSpongeParams>,
    SpongeMode::Overwrite
>;

}

#endif
