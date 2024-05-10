/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.math.BigInteger
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertNull

class PastaCurvesTest {
    @Test
    fun fieldAdd() {
        val a = BigInteger("2c5a3233336a186012edd7a62943cf0ae38a93b9454d5791b9825d4531fbf11c", 16)
        val b = BigInteger("34a99c1d1ad68aeb1d35bcf74ddb040b86ba0a05331200ef3e995b42c73be34a", 16)
        val c = PallasFieldElement(BigInteger("2103ce504e40a34b3023949d771ed31647fe04c26f125f655eee879af937d465", 16))
        val d = VestaFieldElement(BigInteger("2103ce504e40a34b3023949d771ed31647fe04c26ecaafa36bd4cd66f937d465", 16))
        assertEquals(c, PallasFieldElement(a) + PallasFieldElement(b))
        assertEquals(d, VestaFieldElement(a) + VestaFieldElement(b))
        assertEquals(c, PallasField.ZERO + c)
        assertEquals(d, d + VestaField.ZERO)
        assertEquals(PallasField.ONE, PallasField.ONE + PallasField.ZERO)
        assertEquals(VestaField.ONE, VestaField.ZERO + VestaField.ONE)
    }

    @Test
    fun fieldMul() {
        val a = BigInteger("11640cdb3d3a126dabde403009808a4cae45ec00ffac7480d80ac9142abb607f", 16)
        val b = BigInteger("a5111b1ee7f41260df2a030fc99d5aa095ae34332a190ba7ca6d9b54a5d1c85", 16)
        val c = PallasFieldElement(BigInteger("b5842e91b2c5b9b253f653330dcf9d57d1d745479140a959684c13a5a25b6e6", 16))
        val d = VestaFieldElement(BigInteger("158030f7f4f7138ea54d0e0a8797e99ee4c3526ef9c67ccede788174b1f2172", 16))
        assertEquals(c, PallasFieldElement(a) * PallasFieldElement(b))
        assertEquals(d, VestaFieldElement(a) * VestaFieldElement(b))
        assertEquals(PallasField.ZERO, PallasField.ZERO * c)
        assertEquals(VestaField.ZERO, d * VestaField.ZERO)
        assertEquals(c, c * PallasField.ONE)
        assertEquals(d, VestaField.ONE * d)
    }

    @Test
    fun fieldSub() {
        val a = BigInteger("63c6fa6bc7df187ee00659a73a97b1589892a4ae753fe00c7b3764ddd663cd2", 16)
        val b = BigInteger("20ac2a42b38f940e1bdc81e7b258588c04aee2f11a782e579033601a00df0730", 16)
        val c = PallasFieldElement(BigInteger("2590456408ee5d79d223e3b2c1512289a720e055d628c8c4d0ad4720dc8735a3", 16))
        val d = VestaFieldElement(BigInteger("2590456408ee5d79d223e3b2c1512289a720e055d6707886c3c70154dc8735a3", 16))
        assertEquals(c, PallasFieldElement(a) - PallasFieldElement(b))
        assertEquals(d, VestaFieldElement(a) - VestaFieldElement(b))
        assertEquals(c, c - PallasField.ZERO)
        assertEquals(d, d - VestaField.ZERO)
        assertEquals(PallasField.ZERO, PallasField.ONE - PallasField.ONE)
        assertEquals(VestaField.ZERO, VestaField.ONE - VestaField.ONE)
    }

    @Test
    fun fieldDiv() {
        val a = BigInteger("3faced132f5641f57b1162d06ed827d8ca9fa69f0c7b14822818eef4db6f6fdc", 16)
        val b = BigInteger("152d43a9a19991aa7f8c98ed185a79eda9b2562e4c456bb554c0c0d4d0362904", 16)
        val c = PallasFieldElement(BigInteger("3112d3dbd9cb47dd10c20edd49686b9713d5160fb2560360acc84d06bada7442", 16))
        val d = VestaFieldElement(BigInteger("e1fd01ec64fffe6a6fc237d1608308ddaa1efcb579ea243a347caaf8778061c", 16))
        assertEquals(c, PallasFieldElement(a) / PallasFieldElement(b))
        assertEquals(d, VestaFieldElement(a) / VestaFieldElement(b))
        assertEquals(PallasField.ZERO, PallasField.ZERO / c)
        assertFailsWith<ArithmeticException> { d / VestaField.ZERO }
        assertEquals(PallasField.ONE, PallasField.ONE / PallasField.ONE)
        assertEquals(d, d / VestaField.ONE)
    }

    @Test
    fun fieldNeg() {
        val a = BigInteger("12610bc44a0bbc319a91fc24e99a98ef2bd29a2b535bbd1a74bc100a698e34fa", 16)
        val b = PallasFieldElement(BigInteger("2d9ef43bb5f443ce656e03db16656710f673fed0b5f13c01247120e29671cb07", 16))
        val c = VestaFieldElement(BigInteger("2d9ef43bb5f443ce656e03db16656710f673fed0b638ebc3178adb169671cb07", 16))
        assertEquals(b, -PallasFieldElement(a))
        assertEquals(c, -VestaFieldElement(a))
        assertEquals(PallasField.ZERO, -PallasField.ZERO)
        assertEquals(VestaField.ZERO, -VestaField.ZERO)
        assertEquals(PallasFieldElement(PallasField.order - BigInteger.ONE), -PallasField.ONE)
        assertEquals(VestaFieldElement(VestaField.order - BigInteger.ONE), -VestaField.ONE)
    }

    @Test
    fun fieldInv() {
        val a = PallasFieldElement(BigInteger("f34fe2fd15703dc7eba4a68d48fa9ee0e9ab8746f759eb8fc23828a4aa48900", 16))
        val b = PallasFieldElement(BigInteger("87f2909b3c53a656a9f0f126b8458afa89ececeb5676d93c9d4594c4aacc34d", 16))
        assertEquals(a, b.inv())
        assertEquals(b, a.inv())
        assertFailsWith<ArithmeticException> { VestaField.ZERO.inv() }
    }

    @Test
    fun fieldSqrt() {
        val a = BigInteger("35aeb661a5f2e7df341a8f256036c025e07b8e45002f7d9da0c8f7b5aa744aea", 16)
        val b = PallasFieldElement(BigInteger("344a642baaa8f21985d0757617709370cdc5b87574ecd97b4cf3c9d915689609", 16))
        val c = BigInteger("39fce7dbf35569b5dc603860e3254bf9e61e3b57ba958a05a121b318906fe126", 16)
        val d = VestaFieldElement(BigInteger("2fd1206ca31cb1de80ffb18d6b4e5095edafca2beb056dfe0125bf1e0cae890a", 16))
        assertEquals(b, PallasFieldElement(a).sqrt())
        assertNull(VestaFieldElement(a).sqrt())
        assertEquals(d, VestaFieldElement(c).sqrt())
        assertNull(PallasFieldElement(c).sqrt())
        assertEquals(PallasField.ZERO, PallasField.ZERO.sqrt())
        assertEquals(VestaField.ZERO, VestaField.ZERO.sqrt())
        assertEquals(PallasField.ONE, PallasField.ONE.sqrt())
        assertEquals(VestaField.ONE, VestaField.ONE.sqrt())
    }

    @Test
    fun groupNegAffine() {
        val a = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("2c998f5cd6f89a5323244238dcb0e122f3c48b690d17895d64c622fe7b134872", 16)),
            PallasFieldElement(BigInteger("2376d983140e67283c34cb1b20d3a6889b55892b51c224c059ba1f97a768959b", 16)),
        )
        val b = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("2c998f5cd6f89a5323244238dcb0e122f3c48b690d17895d64c622fe7b134872", 16)),
            PallasFieldElement(BigInteger("1c89267cebf198d7c3cb34e4df2c597786f10fd0b78ad45b3f73115558976a66", 16)),
        )
        val c = VestaGroupElementAffine(
            VestaFieldElement(BigInteger("2b84f575fc91b8f506713c696425fd86ea1f134bdb0f2821816f00ab1eeeaa92", 16)),
            VestaFieldElement(BigInteger("179180e8abc3d15ed1d6bc287b7debe66b7c386cad750458ad956514255556bd", 16)),
        )
        val d = VestaGroupElementAffine(
            VestaFieldElement(BigInteger("2b84f575fc91b8f506713c696425fd86ea1f134bdb0f2821816f00ab1eeeaa92", 16)),
            VestaFieldElement(BigInteger("286e7f17543c2ea12e2943d784821419b6ca608f5c1fa484deb1860cdaaaa944", 16)),
        )
        assertEquals(b, -a)
        assertEquals(d, -c)
        assertEquals(PallasGroup.INFINITY_AFFINE, -PallasGroup.INFINITY_AFFINE)
        assertEquals(VestaGroup.INFINITY_AFFINE, -VestaGroup.INFINITY_AFFINE)
    }

    @Test
    fun groupAddAffine() {
        val a = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("1e3dbd8ef7121f586a32c8789be6c1bd516ea0b7b5e00d356527f3b9137c7f13", 16)),
            PallasFieldElement(BigInteger("c09c8b193a30e6989afa1cd8e3f468529cc2294b5111c80dc53080d10a133e3", 16)),
        )
        val b = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("172c422e616dc9017cb392143dcdb133e1071d8e87806ccd9b222d82665aac69", 16)),
            PallasFieldElement(BigInteger("fb0e51efc9e8cd9c0a70e8fa507ec59fcb5da21d8cac79550c4f98d1dc2d362", 16)),
        )
        val c = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("3105fd2e4cf209b0db4e0e0772661ffaee9083b4e5faac71251d9ddbf05c2f23", 16)),
            PallasFieldElement(BigInteger("67e082d0d17fffdd4de37c218a55e188dbb09200621dad577fab3b592cf9ef4", 16)),
        )
        val d = VestaGroupElementAffine(
            VestaFieldElement(BigInteger("3d3b0ea90d13082aa6862f0dac1e211c286614f222bafe7210862d448ef0e466", 16)),
            VestaFieldElement(BigInteger("2b63efb469e111e71293b98fbe5008688cb8de0ca571a0075ea200e74abca6f9", 16)),
        )
        val e = VestaGroupElementAffine(
            VestaFieldElement(BigInteger("1f85aa11a81f4464c19b28e5c55ace5b51689ef25f63156cce7d59e28969a639", 16)),
            VestaFieldElement(BigInteger("3816248bb82336b770bc06e56883e8fa92c4557f4b16f1ab9fbd831db7750df8", 16)),
        )
        val f = VestaGroupElementAffine(
            VestaFieldElement(BigInteger("1f85aa11a81f4464c19b28e5c55ace5b51689ef25f63156cce7d59e28969a639", 16)),
            VestaFieldElement(BigInteger("7e9db7447dcc9488f43f91a977c17058f82437cbe7db731ec896803488af209", 16)),
        )
        assertEquals(c, a + b)
        assertEquals(c, b + a)
        assertEquals(e, d + d)
        assertEquals(VestaGroup.INFINITY_AFFINE, e + f)
        assertEquals(c, PallasGroup.INFINITY_AFFINE + c)
        assertEquals(c, c + PallasGroup.INFINITY_AFFINE)
        assertEquals(VestaGroup.INFINITY_AFFINE, VestaGroup.INFINITY_AFFINE + VestaGroup.INFINITY_AFFINE)
    }

    @Test
    fun groupMulAffine() {
        val a = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("3aed134ed42ad34f18db7529fb0ed4470dbb0a157d676eca74f7789208b87676", 16)),
            PallasFieldElement(BigInteger("2a7a1566f8a07fe9bc87e23a8556e2e144afbe659053d2bfcbbaaa5a42ed856b", 16)),
        )
        val b = VestaFieldElement(BigInteger("e18ddb951f8a3a10c33028e6cd15a9b4480c3c825f515b6da24b75e7c813623", 16))
        val c = PallasGroupElementAffine(
            PallasFieldElement(BigInteger("2a0da0b30d7ff6d2956f3eeb2f72dc75310b85f70aa9123640ed78f1b6c3ff03", 16)),
            PallasFieldElement(BigInteger("2ddbebbf3c0412bc46ffaec08aaebc3c6bd717f3205bb841814983d016f79ec0", 16)),
        )
        val d = PallasFieldElement(BigInteger("251d364ed569cbf14184665ce3fa321e9678002959e04609d1a0ecc692cee9e1", 16))
        assertEquals(c, a * b)
        assertEquals(a, a * VestaField.ONE)
        assertEquals(PallasGroup.INFINITY_AFFINE, a * VestaField.ZERO)
        assertEquals(VestaGroup.INFINITY_AFFINE, VestaGroup.INFINITY_AFFINE * d)
    }

    @Test
    fun groupNegProjective() {
        val a = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5", 16)),
            PallasFieldElement(BigInteger("2f89c29d9ae36f7c0f20ef5d73f85cea5fdc1cfeae3b96e36c377d3b2f1afb4d", 16)),
            PallasField.ONE,
        )
        val b = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("1c92e421c15f698f5f595eb458e7ce36f9fa43fc4d06591aacd1658a92722cd5", 16)),
            PallasFieldElement(BigInteger("10763d62651c9083f0df10a28c07a315c26a7bfd5b1162382cf5b3b1d0e504b4", 16)),
            PallasField.ONE,
        )
        val c = VestaGroupElementProjective(
            VestaFieldElement(BigInteger("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17", 16)),
            VestaFieldElement(BigInteger("e2e3683b3e12f2b986560a0b3a208f29066185aad807056b440e687f990a70a", 16)),
            VestaField.ONE,
        )
        val d = VestaGroupElementProjective(
            VestaFieldElement(BigInteger("29ccc7054c866d02883e099de5420e2bd07ca59ebc8f1901696496382d2b1c17", 16)),
            VestaFieldElement(BigInteger("31d1c97c4c1ed0d4679a9f5f4c5df70d91e080a15c143886d8060499066f58f7", 16)),
            VestaField.ONE,
        )
        assertEquals(b, -a)
        assertEquals(d, -c)
        assertEquals(PallasGroup.INFINITY_PROJECTIVE, -PallasGroup.INFINITY_PROJECTIVE)
        assertEquals(VestaGroup.INFINITY_PROJECTIVE, -VestaGroup.INFINITY_PROJECTIVE)
    }

    @Test
    fun groupAddProjective() {
        val a = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("248949bf1e33e577c48df9037c0fedce42ea070f91125cd796f49349a994794c", 16)),
            PallasFieldElement(BigInteger("1a384ee0cd22f32777ff370d3ed17b85b3837a61f7c3c9d3097f06799303d15", 16)),
            PallasField.ONE,
        )
        val b = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("342cfacf5781efbb03d6326015c9078aac0fbc7e5f17d6ad71c9bd8d5bb0e41c", 16)),
            PallasFieldElement(BigInteger("37fd32ff6401ce86774f1b494ee915cec66be45e02981274e16e725eedf671ba", 16)),
            PallasField.ONE,
        )
        val c = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("201da427944269dee8b83e3cb8400f980a26ca9b89e6787e97c70ab09460d2e", 16)),
            PallasFieldElement(BigInteger("1d7929dcd5888af7651396fbcf1c145e178f5cbbbc9f497496c9b531692df787", 16)),
            PallasField.ONE,
        )
        val d = VestaGroupElementProjective(
            VestaFieldElement(BigInteger("2e3f99264efffdf2e6a620de2fd553baadc50da215ba7d2cace02a1843cab60e", 16)),
            VestaFieldElement(BigInteger("3076516f0a8d132db8e5d71e15f1455c39b6cffa67946cd15b5daeb331557ba4", 16)),
            VestaField.ONE,
        )
        val e = VestaGroupElementProjective(
            VestaFieldElement(BigInteger("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8", 16)),
            VestaFieldElement(BigInteger("1d858f8d8cbea25bc870538280467c2ca440be332b2e33860552b61476160a0", 16)),
            VestaField.ONE,
        )
        val f = VestaGroupElementProjective(
            VestaFieldElement(BigInteger("20902e52296c05a0a09ef0150af8bafe836cb1f934f0325abc1afbebc93c09c8", 16)),
            VestaFieldElement(BigInteger("3e27a707273415da4378fac7d7fb983d58028d18d6e1c5a52bf1bfbfb89e9f61", 16)),
            VestaField.ONE,
        )
        assertEquals(c, a + b)
        assertEquals(c, b + a)
        assertEquals(e, d + d)
        assertEquals(VestaGroup.INFINITY_PROJECTIVE, e + f)
        assertEquals(c, PallasGroup.INFINITY_PROJECTIVE + c)
        assertEquals(c, c + PallasGroup.INFINITY_PROJECTIVE)
        assertEquals(VestaGroup.INFINITY_PROJECTIVE, VestaGroup.INFINITY_PROJECTIVE + VestaGroup.INFINITY_PROJECTIVE)
    }

    @Test
    fun groupMulProjective() {
        val a = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("1cb441132f1df394ea0b892518b5f8143814ca5afb8bfcb2cd0b8eaba568b29c", 16)),
            PallasFieldElement(BigInteger("1b01d848ea1769e4e319244446ceebeab80d1687ecd75e1191f8c158a02aaec6", 16)),
            PallasField.ONE,
        )
        val b = VestaFieldElement(BigInteger("27d286de826c7abc89876e85217410148a67ed053968ac6d326ae99eeb11d7f1", 16))
        val c = PallasGroupElementProjective(
            PallasFieldElement(BigInteger("3ae71da7c530d0bbb097cc6b688bb849d1ee146e167637e27486eb874a015ded", 16)),
            PallasFieldElement(BigInteger("101f7a91b0e870b0626c7234eb0024120b66bd06109e55f892fdd00bd5192419", 16)),
            PallasField.ONE,
        )
        val d = PallasFieldElement(BigInteger("8f41a93bb8c52e757404c04e2519c5f66b126176b9f7307de457606b2be8946", 16))
        assertEquals(c, a * b)
        assertEquals(a, a * VestaField.ONE)
        assertEquals(PallasGroup.INFINITY_PROJECTIVE, a * VestaField.ZERO)
        assertEquals(VestaGroup.INFINITY_PROJECTIVE, VestaGroup.INFINITY_PROJECTIVE * d)
    }
}
