/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.KInput
import kotlinx.serialization.KOutput
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer

@Serializable
class SerializableByteArray(val array: ByteArray) {
    constructor(size: Int) : this(ByteArray(size))

    fun size(): Int = array.size

    @Serializer(forClass = SerializableByteArray::class)
    companion object {
        override fun save(output: KOutput, obj: SerializableByteArray) {
            (output as BlacknetOutput).writeSerializableByteArrayValue(obj)
        }

        override fun load(input: KInput): SerializableByteArray {
            return (input as BlacknetInput).readSerializableByteArrayValue()
        }
    }
}