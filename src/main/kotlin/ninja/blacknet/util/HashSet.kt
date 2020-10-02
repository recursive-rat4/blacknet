/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.util.Collections
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.SetSerializer
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

inline fun <T, E> makeSet(content: Array<out E>, factory: () -> T): T
    where T : MutableSet<E> = factory().apply { addAll(content) }

fun <E> hashSetOf(vararg content: E) =
    makeSet<HashSet<E>, E>(content) { HashSet(expectedSize = content.size) }

@Serializable(with = HashSetSerializer::class)
open class HashSet<T>(map: HashMap<T, Boolean> = HashMap<T, Boolean>()) : MutableSet<T> by Collections.newSetFromMap(map) {
    constructor(expectedSize: Int) : this(HashMap<T, Boolean>(expectedSize = expectedSize))
}

/**
 * Serializes a [HashSet].
 */
class HashSetSerializer<E>(
        elementSerializer: KSerializer<E>
) : SetSerializer<HashSet<E>, E>(
        elementSerializer
) {
    override val descriptor: SerialDescriptor = ListSerialDescriptor(
            "ninja.blacknet.util.HashSetSerializer",
            elementSerializer.descriptor
    )

    override fun factory() = HashSet<E>()
    override fun factory(size: Int) = HashSet<E>(expectedSize = size)
}
