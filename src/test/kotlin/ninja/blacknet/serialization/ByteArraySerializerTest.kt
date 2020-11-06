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
import ninja.blacknet.crypto.HashEncoder
import ninja.blacknet.rpc.requests.RequestFormat
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.json.json
import ninja.blacknet.util.plus

class ByteArraySerializerTest {
    private val byteArray = ByteArray(16) { it.toByte() }
    private val binaryEncoded = 144.toByte() + byteArray
    private val hexEncoded = "000102030405060708090A0B0C0D0E0F"
    private val jsonEncoded = "\"$hexEncoded\""

    @Test
    fun binaryDecoder() {
        assertEquals(byteArray, binaryFormat.decodeFromByteArray(ByteArraySerializer, binaryEncoded))
    }

    @Test
    fun binaryEncoder() {
        assertEquals(binaryEncoded, binaryFormat.encodeToByteArray(ByteArraySerializer, byteArray))
    }

    @Test
    fun hashCoder() {
        assertEquals(16, HashEncoder.buildHash("MD5") { encodeSerializableValue(ByteArraySerializer, byteArray) }.size)
    }

    @Test
    fun jsonDecoder() {
        assertEquals(byteArray, json.decodeFromString(ByteArraySerializer, jsonEncoded))
    }

    @Test
    fun jsonEncoder() {
        assertTrue(
            json.encodeToString(ByteArraySerializer, byteArray)
            .compareTo(jsonEncoded, ignoreCase = true) == 0
        )
    }

    @Test
    fun requestDecoder() {
        @Serializable
        class Request(@Serializable(with = ByteArraySerializer::class) val byteArray: ByteArray)
        assertEquals(
                byteArray,
                RequestFormat(
                        serializersModuleOf(ByteArraySerializer::class, ByteArrayAsStringSerializer)
                ).decodeFromParameters(
                        Request.serializer(),
                        parametersOf("byteArray", hexEncoded)
                ).byteArray)
    }
}
