/*
 * Copyright (c) 2023-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlin.test.Test
import kotlin.test.assertEquals
import ninja.blacknet.io.inputStream
import ninja.blacknet.io.outputStream
import ninja.blacknet.network.I2PSAM.Companion.readUTF8Line
import ninja.blacknet.network.I2PSAM.Companion.writeStringUtf8
import ninja.blacknet.util.byteArrayOfInts

class I2PSAMTest {
    @Test
    fun destinationHash() {
        assertEquals(
            byteArrayOfInts(81, 169, 239, 153, 149, 11, 34, 49, 163, 77, 41, 180, 244, 162, 252, 194, 49, 92, 204, 43, 2, 56, 105, 63, 140, 102, 235, 132, 22, 244, 63, 19),
            I2PSAM.hash("EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttd3bv4RZ3HHk0U1v2T5r8N6TFmPNsTli1XzmB20yGQHW4BQAEAAcAAA==")
        )
    }

    @Test
    fun utf8() {
        val nezumi = "ネズミ"
        val bytes = byteArrayOfInts(
            0xE3, 0x83, 0x8D,
            0xE3, 0x82, 0xBA,
            0xE3, 0x83, 0x9F,
        )
        assertEquals(
            nezumi,
            (bytes + 0x0A).inputStream().readUTF8Line()
        )
        assertEquals(
            bytes,
            ByteArray(bytes.size).apply {
                outputStream().writeStringUtf8(nezumi)
            }
        )
    }
}
