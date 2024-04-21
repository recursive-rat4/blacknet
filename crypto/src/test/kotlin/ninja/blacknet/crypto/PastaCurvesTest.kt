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
}
