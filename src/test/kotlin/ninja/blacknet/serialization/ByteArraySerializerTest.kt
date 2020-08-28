/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import io.ktor.http.parametersOf
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.util.plus

class ByteArraySerializerTest {
    private val byteArray = ByteArray(16) { it.toByte() }
    private val binaryEncoded = 144.toByte() + byteArray
    private val hexEncoded = "000102030405060708090A0B0C0D0E0F"
    private val jsonEncoded = "\"$hexEncoded\""

    @Test
    fun binaryDecoder() {
        assertEquals(BinaryDecoder(binaryEncoded).decode(ByteArraySerializer), byteArray)
    }

    @Test
    fun binaryEncoder() {
        assertEquals(BinaryEncoder.toBytes(ByteArraySerializer, byteArray), binaryEncoded)
    }

    @Test
    fun hashCoder() {
        assertEquals(HashCoder.buildHash("MD5") { encodeSerializableValue(ByteArraySerializer, byteArray) }.size, 16)
    }

    @Test
    fun jsonDecoder() {
        assertEquals(Json.parse(ByteArraySerializer, jsonEncoded), byteArray)
    }

    @Test
    fun jsonEncoder() {
        assertTrue(
            Json.stringify(ByteArraySerializer, byteArray)
            .compareTo(jsonEncoded, ignoreCase = true) == 0
        )
    }

    @Test
    fun requestDecoder() {
        @Serializable
        class Request(@Serializable(with = ByteArraySerializer::class) val byteArray: ByteArray)
        assertEquals(RequestDecoder(parametersOf("byteArray", hexEncoded)).decode(Request.serializer()).byteArray, byteArray)
    }
}
