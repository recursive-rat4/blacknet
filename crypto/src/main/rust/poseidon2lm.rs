/*
 * Copyright (c) 2025 Pavel Vasin
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
use crate::lm::LMField;
use crate::permutation::Permutation;
use crate::poseidon2::{Poseidon2Params, Poseidon2Plain};

pub struct Poseidon2LM12 {}

impl Poseidon2Params<LMField, 12, 48, 26, 48> for Poseidon2LM12 {
    const ALPHA: usize = 5;
    const RCB: [LMField; 48] = unsafe {
        [
            LMField::from_unchecked(0x0347a3a42bc10b67),
            LMField::from_unchecked(0x015ab29a7ccf3b45),
            LMField::from_unchecked(0x08df73de08f9679d),
            LMField::from_unchecked(0x0bf4ff419e5bff04),
            LMField::from_unchecked(0x063fab4687ffbfba),
            LMField::from_unchecked(0x097ccc17fd557e10),
            LMField::from_unchecked(0x06158bf2b7063d57),
            LMField::from_unchecked(0x01ccaf620eae9bae),
            LMField::from_unchecked(0x00f0f14796e330e6),
            LMField::from_unchecked(0x085b45937ccef547),
            LMField::from_unchecked(0x052416ae63c15ac7),
            LMField::from_unchecked(0x0cbe7b1be0a5e0ad),
            LMField::from_unchecked(0x0aecd22e8b9fe511),
            LMField::from_unchecked(0x0c285dc416113829),
            LMField::from_unchecked(0x03f96f04edaa17d2),
            LMField::from_unchecked(0x0e108654422580b4),
            LMField::from_unchecked(0x03e1c37b5ce854a8),
            LMField::from_unchecked(0x05174a5494c8ab9c),
            LMField::from_unchecked(0x00a90c6c6617cd01),
            LMField::from_unchecked(0x081db31461eac7a0),
            LMField::from_unchecked(0x05b6660004b4d62b),
            LMField::from_unchecked(0x0088b8b70f959f77),
            LMField::from_unchecked(0x03deec77e82237e0),
            LMField::from_unchecked(0x0278775dcc9ffca6),
            LMField::from_unchecked(0x07096afc77c9dd91),
            LMField::from_unchecked(0x0b5edafeb26d9eb0),
            LMField::from_unchecked(0x0ccc5bdc06bc9125),
            LMField::from_unchecked(0x0d94b32b9ce60856),
            LMField::from_unchecked(0x033d3d7bc3e9cdbf),
            LMField::from_unchecked(0x0e8aeb1295bbb658),
            LMField::from_unchecked(0x05bc9138d981911c),
            LMField::from_unchecked(0x08e6b78cabe089fd),
            LMField::from_unchecked(0x04a50210fbeaee12),
            LMField::from_unchecked(0x0d9b1720e311b6c8),
            LMField::from_unchecked(0x09d3c522ecc79b6e),
            LMField::from_unchecked(0x0ae88362a1881354),
            LMField::from_unchecked(0x04b32a28a8019d95),
            LMField::from_unchecked(0x0e2ef45d8fcc99ab),
            LMField::from_unchecked(0x03b420150db162cc),
            LMField::from_unchecked(0x092129bdb9b452d0),
            LMField::from_unchecked(0x06aa9a7359e2c941),
            LMField::from_unchecked(0x0ff62ac7fa8d6159),
            LMField::from_unchecked(0x0c338ae9e9d38445),
            LMField::from_unchecked(0x0ac3b0fd8d759d84),
            LMField::from_unchecked(0x01d1c637868ba096),
            LMField::from_unchecked(0x0f844f90ca1e2c48),
            LMField::from_unchecked(0x062e52b971a14d9e),
            LMField::from_unchecked(0x04b89b80d5ee0d18),
        ]
    };
    const RCP: [LMField; 26] = unsafe {
        [
            LMField::from_unchecked(0x0f00612665a38371),
            LMField::from_unchecked(0x0f2e363b48d2a838),
            LMField::from_unchecked(0x03e5fbeb4884e253),
            LMField::from_unchecked(0x00b4727f153f507b),
            LMField::from_unchecked(0x083b4c7603e626d1),
            LMField::from_unchecked(0x0f15337f342c6d0d),
            LMField::from_unchecked(0x06e3a4a2d7814ad2),
            LMField::from_unchecked(0x0491484b4b43d3c8),
            LMField::from_unchecked(0x06d985632f4ee5b9),
            LMField::from_unchecked(0x0910719e9e80a214),
            LMField::from_unchecked(0x069763dd5c9014db),
            LMField::from_unchecked(0x00ce7a09ab823434),
            LMField::from_unchecked(0x0226828c0b911b23),
            LMField::from_unchecked(0x01a31859237ee406),
            LMField::from_unchecked(0x092a9c146ba096a1),
            LMField::from_unchecked(0x0e79063a53b5ed86),
            LMField::from_unchecked(0x03c981041c202491),
            LMField::from_unchecked(0x0c00bf4065aae2fb),
            LMField::from_unchecked(0x0dbb221b19bb99e1),
            LMField::from_unchecked(0x0095cb65e8ff7cde),
            LMField::from_unchecked(0x037975947c40af88),
            LMField::from_unchecked(0x0585e87b02bb344e),
            LMField::from_unchecked(0x01355721322bd6a8),
            LMField::from_unchecked(0x0ed619e9b67fc6c9),
            LMField::from_unchecked(0x039d3ba1378f08e0),
            LMField::from_unchecked(0x035faf10c7d096dd),
        ]
    };
    const RCE: [LMField; 48] = unsafe {
        [
            LMField::from_unchecked(0x00ade08164391c45),
            LMField::from_unchecked(0x0b612281c2c45b0f),
            LMField::from_unchecked(0x0b446fe9c4117204),
            LMField::from_unchecked(0x0f925d82de88c34d),
            LMField::from_unchecked(0x06e129be9b696152),
            LMField::from_unchecked(0x0d90ea2d08511481),
            LMField::from_unchecked(0x081e024e513f6100),
            LMField::from_unchecked(0x01d978b5490ecc44),
            LMField::from_unchecked(0x02620c92eaeedc6f),
            LMField::from_unchecked(0x0be8a9a38bea722c),
            LMField::from_unchecked(0x0e00130c96eee355),
            LMField::from_unchecked(0x098c09e9a8e0e06e),
            LMField::from_unchecked(0x044b97256f42aaf4),
            LMField::from_unchecked(0x02a06c7a499c52e9),
            LMField::from_unchecked(0x0a2be50397c39657),
            LMField::from_unchecked(0x0fdd52b8229f93c9),
            LMField::from_unchecked(0x0f7dde25666f477e),
            LMField::from_unchecked(0x06c4ec3166059e8c),
            LMField::from_unchecked(0x0f3f1382c73a3702),
            LMField::from_unchecked(0x0bdcbf6dba2b7007),
            LMField::from_unchecked(0x0ca2fccc8374e194),
            LMField::from_unchecked(0x000c54aea63023a1),
            LMField::from_unchecked(0x02db77a7fada20ea),
            LMField::from_unchecked(0x00abdbdcbc8e2dc2),
            LMField::from_unchecked(0x0ff4cc950cb18cbf),
            LMField::from_unchecked(0x0f558c25cc33590f),
            LMField::from_unchecked(0x04873303ba8c357f),
            LMField::from_unchecked(0x00b6fe42a4c5cd14),
            LMField::from_unchecked(0x01fcc811a8544d78),
            LMField::from_unchecked(0x0c49122120c2c3aa),
            LMField::from_unchecked(0x064a335b30212c17),
            LMField::from_unchecked(0x07f5dbf355f52ea2),
            LMField::from_unchecked(0x04db9c9ddfdd3492),
            LMField::from_unchecked(0x0b8142a4ad8d9423),
            LMField::from_unchecked(0x07856db9f67b5c6f),
            LMField::from_unchecked(0x08ca005295e66b8a),
            LMField::from_unchecked(0x04a5882b7f4121db),
            LMField::from_unchecked(0x063058bcb35f2ddc),
            LMField::from_unchecked(0x0138bd981cd839ee),
            LMField::from_unchecked(0x0d9eb84b93dae11e),
            LMField::from_unchecked(0x00a9ab347b56fbc2),
            LMField::from_unchecked(0x04382f2519106ac9),
            LMField::from_unchecked(0x0fe19bcc34df0bf1),
            LMField::from_unchecked(0x0fa87f9d499e4de7),
            LMField::from_unchecked(0x016488c3bac7afb1),
            LMField::from_unchecked(0x0978ce4259293b09),
            LMField::from_unchecked(0x0a2eb19feac9ff75),
            LMField::from_unchecked(0x02e6e0923518637e),
        ]
    };
    const M: [LMField; 12] = unsafe {
        [
            LMField::from_unchecked(0x00c611137cd29615),
            LMField::from_unchecked(0x08cd78c75d58d0b9),
            LMField::from_unchecked(0x0eb30fcecb7cc0a3),
            LMField::from_unchecked(0x0e1ab1bbf1bb64eb),
            LMField::from_unchecked(0x0233c54f46e9ce61),
            LMField::from_unchecked(0x02df01579b344470),
            LMField::from_unchecked(0x08bd4be9c80b57e1),
            LMField::from_unchecked(0x02bba879c9a6e64b),
            LMField::from_unchecked(0x08c35f03b4000a02),
            LMField::from_unchecked(0x00dd07798d070c50),
            LMField::from_unchecked(0x0fecbde4aa2a6b88),
            LMField::from_unchecked(0x05ab0b314af94313),
        ]
    };
}

impl Permutation for Poseidon2LM12 {
    type Domain = [LMField; 12];

    #[inline]
    fn permute(x: &mut Self::Domain) {
        <Self as Poseidon2Plain<LMField, 12, 48, 26, 48>>::permute(x)
    }
}

pub type DuplexPoseidon2LM = DuplexImpl<LMField, 8, 4, 12, Poseidon2LM12>;

pub struct Poseidon2LM8 {}

impl Poseidon2Params<LMField, 8, 32, 26, 32> for Poseidon2LM8 {
    const ALPHA: usize = 5;
    const RCB: [LMField; 32] = unsafe {
        [
            LMField::from_unchecked(0x07f7974aadc2b252),
            LMField::from_unchecked(0x028040cb5a6631ef),
            LMField::from_unchecked(0x06f4d35883f38af4),
            LMField::from_unchecked(0x05de0d63467e9175),
            LMField::from_unchecked(0x08a0cf274bdf837c),
            LMField::from_unchecked(0x0507abc9fb954dd4),
            LMField::from_unchecked(0x014694f1c3ce6736),
            LMField::from_unchecked(0x001d3d6dcd5e9861),
            LMField::from_unchecked(0x0d98cfad3ab5bc82),
            LMField::from_unchecked(0x0ce6e5288f3c8b31),
            LMField::from_unchecked(0x0e754ef60bb64fa3),
            LMField::from_unchecked(0x06531a158b498d8c),
            LMField::from_unchecked(0x0b1c78e8dfc18fae),
            LMField::from_unchecked(0x052a7cac2a65742b),
            LMField::from_unchecked(0x04d08cef69aaacd6),
            LMField::from_unchecked(0x05f57ea1623bb249),
            LMField::from_unchecked(0x02234d3915d5be22),
            LMField::from_unchecked(0x0249027e2ab1a75f),
            LMField::from_unchecked(0x083b22739c76253a),
            LMField::from_unchecked(0x0725601b79b3c1fd),
            LMField::from_unchecked(0x0657b2d9d1e55e89),
            LMField::from_unchecked(0x0583e4530df8dcd0),
            LMField::from_unchecked(0x066354f6a8a726a1),
            LMField::from_unchecked(0x0ccc74b12a7ca5b4),
            LMField::from_unchecked(0x04ee9648e11867b8),
            LMField::from_unchecked(0x09c09c4d8a4dc2dd),
            LMField::from_unchecked(0x0015ef774810569d),
            LMField::from_unchecked(0x0b5da32b44160899),
            LMField::from_unchecked(0x0b2535ef3c1528cf),
            LMField::from_unchecked(0x01ba0d1239e49c7b),
            LMField::from_unchecked(0x08e4be7dccb5a728),
            LMField::from_unchecked(0x0e362184c480a37a),
        ]
    };
    const RCP: [LMField; 26] = unsafe {
        [
            LMField::from_unchecked(0x00341c28355bec18),
            LMField::from_unchecked(0x01dc141fd099ed1e),
            LMField::from_unchecked(0x057cf519dae381a0),
            LMField::from_unchecked(0x066e1d4773cb63b9),
            LMField::from_unchecked(0x03a6043ef1604b57),
            LMField::from_unchecked(0x0e807678f57890ec),
            LMField::from_unchecked(0x03c6073f6592404c),
            LMField::from_unchecked(0x0e6dc264039f7281),
            LMField::from_unchecked(0x03c145f74ddfae62),
            LMField::from_unchecked(0x039202181f05cf5d),
            LMField::from_unchecked(0x0254752793371342),
            LMField::from_unchecked(0x0fd0b5f6db1146c0),
            LMField::from_unchecked(0x0675b7b4fff0d410),
            LMField::from_unchecked(0x07ba96cffdde6f55),
            LMField::from_unchecked(0x036d24ffb3ec818f),
            LMField::from_unchecked(0x038b74915e502b6e),
            LMField::from_unchecked(0x071605a4fda6e13b),
            LMField::from_unchecked(0x0e74aaad0a21ca91),
            LMField::from_unchecked(0x00fa474759435da0),
            LMField::from_unchecked(0x0818a3173f8a6b4e),
            LMField::from_unchecked(0x0418f30d1b2ec1db),
            LMField::from_unchecked(0x02619151cb9cd290),
            LMField::from_unchecked(0x0853a3d757e9ce4a),
            LMField::from_unchecked(0x053ab91fc569f5ef),
            LMField::from_unchecked(0x034f749f7c6a69ab),
            LMField::from_unchecked(0x0b83542ab4ae112f),
        ]
    };
    const RCE: [LMField; 32] = unsafe {
        [
            LMField::from_unchecked(0x0821e4e4f636cc36),
            LMField::from_unchecked(0x09ba5ba8f4d851de),
            LMField::from_unchecked(0x099b0b6e6a008f41),
            LMField::from_unchecked(0x016e37b0cea06ceb),
            LMField::from_unchecked(0x0c5a8401997b053e),
            LMField::from_unchecked(0x0d9c19efe3a00a93),
            LMField::from_unchecked(0x08e05471520a22bc),
            LMField::from_unchecked(0x086a093d22fb76d0),
            LMField::from_unchecked(0x011fd071c25f527f),
            LMField::from_unchecked(0x0764a4f004a46e68),
            LMField::from_unchecked(0x0e128a085f5268b6),
            LMField::from_unchecked(0x07e7d90468605a7f),
            LMField::from_unchecked(0x0ce70dedf469aa65),
            LMField::from_unchecked(0x07ac32a9c57e3776),
            LMField::from_unchecked(0x02b342c73f81cca0),
            LMField::from_unchecked(0x0a9ee934a73ffe88),
            LMField::from_unchecked(0x0cefdd230983cb6c),
            LMField::from_unchecked(0x03f62652033aeb2f),
            LMField::from_unchecked(0x0f51b1c2a92982f3),
            LMField::from_unchecked(0x09a4df350df79ee0),
            LMField::from_unchecked(0x08520280651743cc),
            LMField::from_unchecked(0x0bd98b6f40854736),
            LMField::from_unchecked(0x0d60c030b047923e),
            LMField::from_unchecked(0x0a4c604832b8c8c9),
            LMField::from_unchecked(0x0d368730ba486e29),
            LMField::from_unchecked(0x0f17016bac376bc6),
            LMField::from_unchecked(0x009c0117f562cf6e),
            LMField::from_unchecked(0x017beab6a0564f54),
            LMField::from_unchecked(0x023683c2a3137331),
            LMField::from_unchecked(0x09eccb0c7d0369a6),
            LMField::from_unchecked(0x0f4020ea289d82ac),
            LMField::from_unchecked(0x0416b510243cdd34),
        ]
    };
    const M: [LMField; 8] = unsafe {
        [
            LMField::from_unchecked(0x036c200cb6b515f3),
            LMField::from_unchecked(0x088d23ad1092a292),
            LMField::from_unchecked(0x0297d7f44d0e5254),
            LMField::from_unchecked(0x019dcd142c5815e5),
            LMField::from_unchecked(0x0b9b9e9cfbbf026b),
            LMField::from_unchecked(0x0b6ba17366400885),
            LMField::from_unchecked(0x05f3c627ccefd66a),
            LMField::from_unchecked(0x00c1a1c1706738c3),
        ]
    };
}

impl Permutation for Poseidon2LM8 {
    type Domain = [LMField; 8];

    #[inline]
    fn permute(x: &mut Self::Domain) {
        <Self as Poseidon2Plain<LMField, 8, 32, 26, 32>>::permute(x)
    }
}

pub type JivePoseidon2LM = Jive<LMField, 4, 8, Poseidon2LM8>;
