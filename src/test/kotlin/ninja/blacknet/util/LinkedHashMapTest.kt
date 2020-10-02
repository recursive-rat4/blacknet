/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.json.json

class LinkedHashMapTest {
    @Test
    fun int() {
        val map = LinkedHashMap<Int, Int>()
        assertEquals(0, map.size)
        map.put(4, 16)
        assertEquals(1, map.size)
        map.put(4, 16)
        assertEquals(1, map.size)
    }

    @Test
    fun bytearray() {
        val map = LinkedHashMap<ByteArray, ByteArray>()
        assertEquals(0, map.size)
        map.put(ByteArray(4), ByteArray(16))
        assertEquals(1, map.size)
        map.put(ByteArray(4), ByteArray(16))
        assertEquals(1, map.size)
    }

    @Test
    fun serializer() {
        val binaryEncoded = byteArrayOfInts(
                132,
                0, 0, 0, 1, 0, 0, 0, 16,
                0, 0, 0, 2, 0, 0, 0, 16,
                0, 0, 0, 3, 0, 0, 0, 16,
                0, 0, 0, 4, 0, 0, 0, 16,
        )
        val jsonEncoded = "{\"1\":16,\"2\":16,\"3\":16,\"4\":16}"
        val map = linkedHashMapOf<Int, Int>(
            1 to 16,
            2 to 16,
            3 to 16,
            4 to 16,
        )
        val serializer = LinkedHashMapSerializer(Int.serializer(), Int.serializer())
        assertEquals(map, BinaryDecoder(binaryEncoded).decode(serializer))
        assertEquals(binaryEncoded,BinaryEncoder.toBytes(serializer, map))
        assertEquals(map, json.decodeFromString(serializer, jsonEncoded))
        assertEquals(jsonEncoded, json.encodeToString(serializer, map))
    }
}
