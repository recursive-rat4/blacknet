/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import io.ktor.utils.io.bits.Memory
import io.ktor.utils.io.bits.loadIntAt
import io.ktor.utils.io.bits.of
import io.ktor.utils.io.pool.DefaultPool
import kotlinx.serialization.SerializationStrategy
import ninja.blacknet.Runtime
import ninja.blacknet.db.Salt

/**
 * SipHash-2-4 keyed hash function.
 */
object SipHash {
    /**
     * Computes a hash code value with given serializer and value.
     *
     * @param serializer the serialization strategy
     * @param value the object serializable to [HashCoder]
     */
    fun <T> hashCode(serializer: SerializationStrategy<T>, value: T): Int {
        val coder = pool.borrow()
        return try {
            serializer.serialize(coder, value)
            Memory.of(coder.writer.finish()).loadIntAt(4)
        } catch (e: Throwable) {
            coder.writer.reset()
            throw e
        } finally {
            pool.recycle(coder)
        }
    }

    private val pool = object : DefaultPool<HashCoder>(Runtime.availableProcessors) {
        override fun produceInstance(): HashCoder {
            return HashCoder(
                    KeyedHashWriterJvm("SIPHASH-2-4", Salt.salt),
                    charset = null,
                    allowFloatingPointValues = true
            )
        }

        override fun clearInstance(instance: HashCoder): HashCoder {
            return instance
        }

        override fun validateInstance(instance: HashCoder) {

        }

        override fun disposeInstance(instance: HashCoder) {

        }
    }
}
