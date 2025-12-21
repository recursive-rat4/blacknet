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

use crate::duplex::DuplexImpl;
use crate::jive::Jive;
use crate::permutation::Permutation;
use crate::pervushin::PervushinField;
use crate::poseidon2::{Poseidon2Params, Poseidon2Plain};

pub struct Poseidon2Pervushin12 {}

impl Poseidon2Params<PervushinField, 12, 48, 12, 48> for Poseidon2Pervushin12 {
    const ALPHA: usize = 17;
    const RCB: [PervushinField; 48] = unsafe {
        [
            PervushinField::from_unchecked(0x1bea68551d3b015e),
            PervushinField::from_unchecked(0x0e0b69da4246892f),
            PervushinField::from_unchecked(0x06b5d4d6cb619607),
            PervushinField::from_unchecked(0x081fc5cb5ef8f48d),
            PervushinField::from_unchecked(0x0f21d71bf80ce07f),
            PervushinField::from_unchecked(0x150c658aeb227124),
            PervushinField::from_unchecked(0x02ee3e792332e8ae),
            PervushinField::from_unchecked(0x0eebf1b165764cb6),
            PervushinField::from_unchecked(0x115e45e6fb6a45d3),
            PervushinField::from_unchecked(0x0c57a09021cb05c9),
            PervushinField::from_unchecked(0x10d088963861993f),
            PervushinField::from_unchecked(0x07f45d080b68141c),
            PervushinField::from_unchecked(0x1822a99f95475dad),
            PervushinField::from_unchecked(0x1ab4a2f699eedacc),
            PervushinField::from_unchecked(0x066d2206cb6b4c66),
            PervushinField::from_unchecked(0x14b73c6cb8748636),
            PervushinField::from_unchecked(0x1d4de783d4bfc073),
            PervushinField::from_unchecked(0x0585bf9dec7f2e80),
            PervushinField::from_unchecked(0x016c1a49f443a3a0),
            PervushinField::from_unchecked(0x12755af4eef518e7),
            PervushinField::from_unchecked(0x08bcf53c6dd5f3bb),
            PervushinField::from_unchecked(0x185b3be9bad5d04a),
            PervushinField::from_unchecked(0x03a987ddb8f7faee),
            PervushinField::from_unchecked(0x06053640fe81014d),
            PervushinField::from_unchecked(0x0e1caae45406582c),
            PervushinField::from_unchecked(0x0e9ea46dfbadc5fc),
            PervushinField::from_unchecked(0x1a790c8c3d4d9e3c),
            PervushinField::from_unchecked(0x0fab33b1612f1d30),
            PervushinField::from_unchecked(0x06c1b98e49b19d6d),
            PervushinField::from_unchecked(0x1a6e50d0ded2508d),
            PervushinField::from_unchecked(0x1761336c02c04916),
            PervushinField::from_unchecked(0x0c7085946a46fd93),
            PervushinField::from_unchecked(0x0c78b181d5e5439c),
            PervushinField::from_unchecked(0x01a2200fa9fd0548),
            PervushinField::from_unchecked(0x1e546a16cbc58ca9),
            PervushinField::from_unchecked(0x1259cfd49ea390a6),
            PervushinField::from_unchecked(0x0e5a10cf4f1af897),
            PervushinField::from_unchecked(0x0b49567f5a959dcb),
            PervushinField::from_unchecked(0x0190905a76353ace),
            PervushinField::from_unchecked(0x19d96459612aceba),
            PervushinField::from_unchecked(0x1554e770785449e1),
            PervushinField::from_unchecked(0x0f0da3e1991fcf97),
            PervushinField::from_unchecked(0x01b6c2c6aa77ddf1),
            PervushinField::from_unchecked(0x05dd6897b427fb66),
            PervushinField::from_unchecked(0x0fc18bcfbd6a418f),
            PervushinField::from_unchecked(0x1f8395f8ff136a32),
            PervushinField::from_unchecked(0x062e5dffec9b6de5),
            PervushinField::from_unchecked(0x146a74cddc8964f5),
        ]
    };
    const RCP: [PervushinField; 12] = unsafe {
        [
            PervushinField::from_unchecked(0x1fb1afcf1de89b25),
            PervushinField::from_unchecked(0x079d0c916c68d8f7),
            PervushinField::from_unchecked(0x15a913b241a35d0b),
            PervushinField::from_unchecked(0x07dfadd0d69e16b0),
            PervushinField::from_unchecked(0x1852b50a9797a25e),
            PervushinField::from_unchecked(0x0eec7299aa17eeb1),
            PervushinField::from_unchecked(0x057f7489ace6eb68),
            PervushinField::from_unchecked(0x184ea926c1e48931),
            PervushinField::from_unchecked(0x0a6b1fb6886cd5e3),
            PervushinField::from_unchecked(0x09c7e2b2aeb59896),
            PervushinField::from_unchecked(0x1eccf3cc9bf16107),
            PervushinField::from_unchecked(0x01c01aa4dcf13ca2),
        ]
    };
    const RCE: [PervushinField; 48] = unsafe {
        [
            PervushinField::from_unchecked(0x0d130cb0c9f4b119),
            PervushinField::from_unchecked(0x0a858d7a070a35e8),
            PervushinField::from_unchecked(0x1702d16b4f6ff971),
            PervushinField::from_unchecked(0x079ce000f1bb981b),
            PervushinField::from_unchecked(0x18d431d955ed8cd7),
            PervushinField::from_unchecked(0x124c3f24e0eafbda),
            PervushinField::from_unchecked(0x1d98c2ecaa2e1c4b),
            PervushinField::from_unchecked(0x07069f9ced0bdf71),
            PervushinField::from_unchecked(0x0d1803105c1d3b6e),
            PervushinField::from_unchecked(0x039d825a73fe9fb9),
            PervushinField::from_unchecked(0x1535687903111fa2),
            PervushinField::from_unchecked(0x0b32c9c6925e4e85),
            PervushinField::from_unchecked(0x140ca45c97ad33ff),
            PervushinField::from_unchecked(0x1d6348ceb4790df0),
            PervushinField::from_unchecked(0x003a1367a070ddc0),
            PervushinField::from_unchecked(0x0a347320b1745926),
            PervushinField::from_unchecked(0x023dffb9c064d735),
            PervushinField::from_unchecked(0x0cb64be6e50dfec3),
            PervushinField::from_unchecked(0x1f7d890cbb6a082b),
            PervushinField::from_unchecked(0x0f0e9eaffbf69edc),
            PervushinField::from_unchecked(0x022ae764c1d21284),
            PervushinField::from_unchecked(0x029acc19950d8a2b),
            PervushinField::from_unchecked(0x191e6c68e1bb80a0),
            PervushinField::from_unchecked(0x1b64b2ae7d158167),
            PervushinField::from_unchecked(0x1222826578ea58be),
            PervushinField::from_unchecked(0x1db68e91a53408f4),
            PervushinField::from_unchecked(0x021d79b6189f4cc8),
            PervushinField::from_unchecked(0x0e7ad0ea76407bcc),
            PervushinField::from_unchecked(0x014ec965b6e96ca3),
            PervushinField::from_unchecked(0x12a8ae29b344eae4),
            PervushinField::from_unchecked(0x0f244cc20e64f0cc),
            PervushinField::from_unchecked(0x1f6eb055b6917e8a),
            PervushinField::from_unchecked(0x182f55d5a18931ff),
            PervushinField::from_unchecked(0x12cf53fdae88c372),
            PervushinField::from_unchecked(0x0436ea3fc4884fcc),
            PervushinField::from_unchecked(0x0e7df543a3bd4e9e),
            PervushinField::from_unchecked(0x0189e162bff1604c),
            PervushinField::from_unchecked(0x1f2b6af8fcbeed27),
            PervushinField::from_unchecked(0x10ff4896d48d6371),
            PervushinField::from_unchecked(0x15966f8d48ddc257),
            PervushinField::from_unchecked(0x08d45cc7f7b5667e),
            PervushinField::from_unchecked(0x05ef0dad2a956bba),
            PervushinField::from_unchecked(0x0d303e520990ed56),
            PervushinField::from_unchecked(0x1be6e3c6a3d65ec2),
            PervushinField::from_unchecked(0x0edf34dcc273ab78),
            PervushinField::from_unchecked(0x1f16d269467d0451),
            PervushinField::from_unchecked(0x1d58532bd1a60771),
            PervushinField::from_unchecked(0x1aed3e535fd6fed7),
        ]
    };
    const M: [PervushinField; 12] = unsafe {
        [
            PervushinField::from_unchecked(0x0eee0c4eb310a7a7),
            PervushinField::from_unchecked(0x0baf08f61c807618),
            PervushinField::from_unchecked(0x0c0aa63ae98dc48a),
            PervushinField::from_unchecked(0x01d1b9764d0e0f78),
            PervushinField::from_unchecked(0x01e24c73473b8561),
            PervushinField::from_unchecked(0x160e8b75670d5b0d),
            PervushinField::from_unchecked(0x1b1663047fb85fd5),
            PervushinField::from_unchecked(0x0be9a348aa0a7d5a),
            PervushinField::from_unchecked(0x03bbf04ccacba145),
            PervushinField::from_unchecked(0x1832a1e74a32e5d2),
            PervushinField::from_unchecked(0x1ea7d86b0bd3d316),
            PervushinField::from_unchecked(0x18db28488c73a35f),
        ]
    };
}

impl Permutation for Poseidon2Pervushin12 {
    type Domain = [PervushinField; 12];

    #[inline]
    fn permute(x: &mut Self::Domain) {
        <Self as Poseidon2Plain<PervushinField, 12, 48, 12, 48>>::permute(x)
    }
}

pub type DuplexPoseidon2Pervushin = DuplexImpl<PervushinField, 8, 4, 12, Poseidon2Pervushin12>;

pub struct Poseidon2Pervushin8 {}

impl Poseidon2Params<PervushinField, 8, 32, 12, 32> for Poseidon2Pervushin8 {
    const ALPHA: usize = 17;
    const RCB: [PervushinField; 32] = unsafe {
        [
            PervushinField::from_unchecked(0x0cb45a089048f620),
            PervushinField::from_unchecked(0x0c10af255db88f5c),
            PervushinField::from_unchecked(0x06493f1d491f60d4),
            PervushinField::from_unchecked(0x173eae52b9d380d3),
            PervushinField::from_unchecked(0x11a5a838544a3c1a),
            PervushinField::from_unchecked(0x07404edcd7e89e92),
            PervushinField::from_unchecked(0x1d40479608c9b9d6),
            PervushinField::from_unchecked(0x06b611851d10b789),
            PervushinField::from_unchecked(0x00354a30fedeb9e1),
            PervushinField::from_unchecked(0x1967922b30e35d36),
            PervushinField::from_unchecked(0x150bb950bbecb278),
            PervushinField::from_unchecked(0x1c04cfadc662c4ee),
            PervushinField::from_unchecked(0x0ad1ed8e7469efb1),
            PervushinField::from_unchecked(0x0c44b22126dd10c5),
            PervushinField::from_unchecked(0x187512642b323393),
            PervushinField::from_unchecked(0x10360a33d92e36e6),
            PervushinField::from_unchecked(0x0172841b8ad2dd36),
            PervushinField::from_unchecked(0x07a5d90f058ec967),
            PervushinField::from_unchecked(0x01b44df13e151be8),
            PervushinField::from_unchecked(0x1ed26d3ae133ed27),
            PervushinField::from_unchecked(0x07e05246dfcca449),
            PervushinField::from_unchecked(0x074b563fe8279746),
            PervushinField::from_unchecked(0x11fbf6fca24fe825),
            PervushinField::from_unchecked(0x1c69601b617b69f8),
            PervushinField::from_unchecked(0x0b17dc8fd2b1f8da),
            PervushinField::from_unchecked(0x002a1e612377e372),
            PervushinField::from_unchecked(0x1723f59722bbb64f),
            PervushinField::from_unchecked(0x07efa3ecf0233197),
            PervushinField::from_unchecked(0x00b311bca34874ce),
            PervushinField::from_unchecked(0x176e2ca7054e8e74),
            PervushinField::from_unchecked(0x0aa822f86df62c82),
            PervushinField::from_unchecked(0x13d1caf98f57d3a6),
        ]
    };
    const RCP: [PervushinField; 12] = unsafe {
        [
            PervushinField::from_unchecked(0x155c592e291d9e97),
            PervushinField::from_unchecked(0x01750c34cc46a1f0),
            PervushinField::from_unchecked(0x0fb211f991a780d5),
            PervushinField::from_unchecked(0x06495472df01645d),
            PervushinField::from_unchecked(0x1219cfb743292363),
            PervushinField::from_unchecked(0x04bb9d7be5e4c0f7),
            PervushinField::from_unchecked(0x09f5cf65399c4301),
            PervushinField::from_unchecked(0x0b5e39ff47dff9b3),
            PervushinField::from_unchecked(0x0d9ee0f26e988c0d),
            PervushinField::from_unchecked(0x0f5e7ae6fae36530),
            PervushinField::from_unchecked(0x11f72bc1bc54b874),
            PervushinField::from_unchecked(0x1613f2be80e12d5f),
        ]
    };
    const RCE: [PervushinField; 32] = unsafe {
        [
            PervushinField::from_unchecked(0x0fd0f9f6fcdc7f49),
            PervushinField::from_unchecked(0x03149cc9f68b55ad),
            PervushinField::from_unchecked(0x0d8aac8351b29ead),
            PervushinField::from_unchecked(0x190f94b6dc9e3d6f),
            PervushinField::from_unchecked(0x1568b50bc4bdbf23),
            PervushinField::from_unchecked(0x16893566cdfd0171),
            PervushinField::from_unchecked(0x14155d8230c76e6f),
            PervushinField::from_unchecked(0x02846b79954cbc77),
            PervushinField::from_unchecked(0x01d7f2dc47d2af30),
            PervushinField::from_unchecked(0x0e2b9ddb4881e796),
            PervushinField::from_unchecked(0x12ace083096ee2d8),
            PervushinField::from_unchecked(0x0c7b101a1a2e52ea),
            PervushinField::from_unchecked(0x07712b54233e515b),
            PervushinField::from_unchecked(0x1511f62ae46e2dc2),
            PervushinField::from_unchecked(0x0c0bb4da075a298f),
            PervushinField::from_unchecked(0x002757a9fdbb4c9e),
            PervushinField::from_unchecked(0x0f5cc55693d423ab),
            PervushinField::from_unchecked(0x15f37ab84b697cbc),
            PervushinField::from_unchecked(0x19c283ac67c1499c),
            PervushinField::from_unchecked(0x0b25c59f02da3791),
            PervushinField::from_unchecked(0x14298fa04d4fe8ed),
            PervushinField::from_unchecked(0x1c3b94b708a44cf1),
            PervushinField::from_unchecked(0x05fc7b2996e4929e),
            PervushinField::from_unchecked(0x1e41e46e28d89b65),
            PervushinField::from_unchecked(0x0d46c336718dde36),
            PervushinField::from_unchecked(0x13eaed2096d0d455),
            PervushinField::from_unchecked(0x01b56caab2dd914e),
            PervushinField::from_unchecked(0x0209f5c5b43d31a6),
            PervushinField::from_unchecked(0x19bfd8a77cbbaa77),
            PervushinField::from_unchecked(0x18563ad7c351001c),
            PervushinField::from_unchecked(0x1c7418fc20672a26),
            PervushinField::from_unchecked(0x1fadbde450f44a00),
        ]
    };
    const M: [PervushinField; 8] = unsafe {
        [
            PervushinField::from_unchecked(0x1e1a6271f0929a74),
            PervushinField::from_unchecked(0x0da614dc89563cbc),
            PervushinField::from_unchecked(0x16d156c6747508bf),
            PervushinField::from_unchecked(0x0a00ce5c502d92e5),
            PervushinField::from_unchecked(0x16919b4feb3a6563),
            PervushinField::from_unchecked(0x1ced093471943ce6),
            PervushinField::from_unchecked(0x15d6ce01acb63062),
            PervushinField::from_unchecked(0x17355fc9a26a7fed),
        ]
    };
}

impl Permutation for Poseidon2Pervushin8 {
    type Domain = [PervushinField; 8];

    #[inline]
    fn permute(x: &mut Self::Domain) {
        <Self as Poseidon2Plain<PervushinField, 8, 32, 12, 32>>::permute(x)
    }
}

pub type JivePoseidon2Pervushin = Jive<PervushinField, 4, 8, Poseidon2Pervushin8>;
