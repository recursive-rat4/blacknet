/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.nio.ByteBuffer
import io.ktor.utils.io.pool.DefaultPool
import kotlin.random.Random
import kotlinx.serialization.SerializationStrategy
import ninja.blacknet.Runtime

/**
 * SipHash-2-4 keyed hash function.
 */
object SipHash {
    private const val KEY_SIZE_BYTES = 16
    private val hashCodeKey = Random.nextBytes(KEY_SIZE_BYTES)

    /**
     * Computes a hash code value with given serializer and value.
     *
     * @param serializer the serialization strategy
     * @param value the object serializable to [HashEncoder]
     */
    fun <T> hashCode(serializer: SerializationStrategy<T>, value: T): Int {
        val coder = pool.borrow()
        return try {
            serializer.serialize(coder, value)
            ByteBuffer.wrap(coder.writer.finish()).getInt(4)
        } catch (e: Throwable) {
            coder.writer.reset()
            throw e
        } finally {
            pool.recycle(coder)
        }
    }

    private val pool = object : DefaultPool<HashEncoder>(Runtime.availableProcessors) {
        override fun produceInstance(): HashEncoder {
            return HashEncoder(
                    KeyedHashWriterJvm("SIPHASH-2-4", hashCodeKey),
                    charset = null,
            )
        }

        override fun clearInstance(instance: HashEncoder): HashEncoder {
            return instance
        }

        override fun validateInstance(instance: HashEncoder) {

        }

        override fun disposeInstance(instance: HashEncoder) {

        }
    }
}
