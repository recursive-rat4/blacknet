/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("EXPERIMENTAL_API_USAGE")

package ninja.blacknet.crypto

import com.rfksystems.blake2b.Blake2b
import io.ktor.utils.io.bits.*
import ninja.blacknet.SystemService
import ninja.blacknet.util.highByte

/**
 * BLAKE2b-256 hash function.
 */
@SystemService
object Blake2b : (ByteArray) -> ByteArray {
    const val DIGEST_SIZE_BYTES = 32
    const val DIGEST_SIZE_BITS = DIGEST_SIZE_BYTES * Byte.SIZE_BITS

    /**
     * Returns a hash with the given [digestSize].
     *
     * @param digestSize the digest size of the hash
     * @return the [ByteArray] containing the hash value
     */
    internal fun hash(digestSize: Int, message: ByteArray): ByteArray {
        val bytes = ByteArray(digestSize / 8)
        val b = Blake2b(digestSize)
        b.update(message, 0, message.size)
        b.digest(bytes, 0)
        return bytes
    }

    /**
     * Builds a hash value with the given [init] builder.
     *
     * @param init the initialization function with the [Hasher] receiver
     * @return the built [Hash] value
     */
    fun hasher(init: Hasher.() -> Unit): Hash {
        val hasher = Hasher()
        hasher.init()
        return hasher.result()
    }

    /**
     * DSL builder for a [Hash] value.
     */
    class Hasher(private val blake2b: Blake2b) {
        internal constructor() : this(Blake2b(DIGEST_SIZE_BITS))

        /**
         * Adds a [byte] value.
         *
         * @param byte the [Byte] containing the data
         */
        fun x(byte: Byte) {
            blake2b.update(byte)
        }

        /**
         * Adds a [short] value in the big-endian byte order.
         *
         * @param short the [Short] containing the data
         */
        fun x(short: Short) {
            short.let {
                blake2b.update(it.highByte)
                blake2b.update(it.lowByte)
            }
        }

        /**
         * Adds an [int] value in the big-endian byte order.
         *
         * @param int the [Int] containing the data
         */
        fun x(int: Int) {
            int.highShort.let {
                blake2b.update(it.highByte)
                blake2b.update(it.lowByte)
            }
            int.lowShort.let {
                blake2b.update(it.highByte)
                blake2b.update(it.lowByte)
            }
        }

        /**
         * Adds a [long] value in the big-endian byte order.
         *
         * @param long the [Long] containing the data
         */
        fun x(long: Long) {
            long.highInt.let {
                it.highShort.let {
                    blake2b.update(it.highByte)
                    blake2b.update(it.lowByte)
                }
                it.lowShort.let {
                    blake2b.update(it.highByte)
                    blake2b.update(it.lowByte)
                }
            }
            long.lowInt.let {
                it.highShort.let {
                    blake2b.update(it.highByte)
                    blake2b.update(it.lowByte)
                }
                it.lowShort.let {
                    blake2b.update(it.highByte)
                    blake2b.update(it.lowByte)
                }
            }
        }

        /**
         * Adds a [string] value using the UTF-8 charset.
         *
         * @param string the [String] containing the data
         */
        fun x(string: String) {
            val bytes = string.toByteArray(Charsets.UTF_8)
            blake2b.update(bytes, 0, bytes.size)
        }

        /**
         * Adds a [bytes] value.
         *
         * @param bytes the [ByteArray] containing the data
         */
        fun x(bytes: ByteArray) {
            blake2b.update(bytes, 0, bytes.size)
        }

        /**
         * Adds some bytes of a [bytes] value.
         *
         * @param bytes the [ByteArray] containing the data
         * @param offset the offset of the data
         * @param length the length of the data
         */
        fun x(bytes: ByteArray, offset: Int, length: Int) {
            blake2b.update(bytes, offset, length)
        }

        /**
         * Adds a [hash] value.
         *
         * @param hash the [Hash] containing the data
         */
        fun x(hash: Hash) {
            blake2b.update(hash.bytes, 0, Hash.SIZE_BYTES)
        }

        /**
         * Adds a [publicKey] value.
         *
         * @param publicKey the [PublicKey] containing the data
         */
        fun x(publicKey: PublicKey) {
            blake2b.update(publicKey.bytes, 0, PublicKey.SIZE_BYTES)
        }

        internal fun result(): Hash {
            val bytes = ByteArray(Hash.SIZE_BYTES)
            blake2b.digest(bytes, 0)
            return Hash(bytes)
        }
    }

    override fun invoke(bytes: ByteArray): ByteArray = hasher { x(bytes) }.bytes
}
