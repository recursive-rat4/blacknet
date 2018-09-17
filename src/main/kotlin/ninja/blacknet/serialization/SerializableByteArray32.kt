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
class SerializableByteArray32(val array: ByteArray) {
    constructor() : this(ByteArray(SIZE))

    @Serializer(forClass = SerializableByteArray32::class)
    companion object {
        const val SIZE = 32

        override fun save(output: KOutput, obj: SerializableByteArray32) {
            (output as BlacknetOutput).writeByteArrayValue(obj.array, SIZE)
        }

        override fun load(input: KInput): SerializableByteArray32 {
            return SerializableByteArray32((input as BlacknetInput).readByteArrayValue(SIZE))
        }
    }
}