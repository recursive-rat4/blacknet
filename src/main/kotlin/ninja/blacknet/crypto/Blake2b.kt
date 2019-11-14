/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import com.rfksystems.blake2b.Blake2b

/**
 * BLAKE2b-256 hash function.
 */
object Blake2b : (ByteArray) -> ByteArray {
    /**
     * Returns a [Hash] of the [ByteArray].
     */
    fun hash(message: ByteArray): Hash {
        val bytes = ByteArray(Hash.SIZE)
        val b = Blake2b(Hash.DIGEST_SIZE)
        b.update(message, 0, message.size)
        b.digest(bytes, 0)
        return Hash(bytes)
    }

    /**
     * Returns a [Hash] of a block of bytes.
     *
     * @param message the [ByteArray] containing the data
     * @param offset the offset of the data
     * @param length the length of the data
     */
    fun hash(message: ByteArray, offset: Int, length: Int): Hash {
        val bytes = ByteArray(Hash.SIZE)
        val b = Blake2b(Hash.DIGEST_SIZE)
        b.update(message, offset, length)
        b.digest(bytes, 0)
        return Hash(bytes)
    }

    /**
     * Returns a hash with given [digestSize].
     */
    fun hash(digestSize: Int, message: ByteArray): ByteArray {
        val bytes = ByteArray(digestSize / 8)
        val b = Blake2b(digestSize)
        b.update(message, 0, message.size)
        b.digest(bytes, 0)
        return bytes
    }

    /**
     * Returns a [Hash] of the [String] using the UTF-8 charset.
     */
    fun hash(string: String): Hash {
        return hash(string.toByteArray(Charsets.UTF_8))
    }

    /**
     * Builds [Hash] with given [init] builder.
     */
    fun hasher(init: Hasher.() -> Unit): Hash {
        val hasher = Hasher()
        hasher.init()
        return hasher.hash()
    }

    /**
     * DSL builder for a [Hash].
     */
    class Hasher internal constructor(private val blake2b: Blake2b = Blake2b(Hash.DIGEST_SIZE)) {
        /**
         * Adds [Byte] value.
         */
        operator fun plus(byte: Byte): Hasher {
            blake2b.update(byte)
            return this
        }

        /**
         * Adds [Short] value in big-endian byte order.
         */
        operator fun plus(short: Short): Hasher {
            blake2b.update((short.toInt() shr 8).toByte())
            blake2b.update(short.toByte())
            return this
        }

        /**
         * Adds [Int] value in big-endian byte order.
         */
        operator fun plus(int: Int): Hasher {
            blake2b.update((int shr 24).toByte())
            blake2b.update((int shr 16).toByte())
            blake2b.update((int shr 8).toByte())
            blake2b.update(int.toByte())
            return this
        }

        /**
         * Adds [Long] value in big-endian byte order.
         */
        operator fun plus(long: Long): Hasher {
            blake2b.update((long shr 56).toByte())
            blake2b.update((long shr 48).toByte())
            blake2b.update((long shr 40).toByte())
            blake2b.update((long shr 32).toByte())
            blake2b.update((long shr 24).toByte())
            blake2b.update((long shr 16).toByte())
            blake2b.update((long shr 8).toByte())
            blake2b.update(long.toByte())
            return this
        }

        /**
         * Adds [String] value using the UTF-8 charset.
         */
        operator fun plus(string: String): Hasher {
            val bytes = string.toByteArray(Charsets.UTF_8)
            blake2b.update(bytes, 0, bytes.size)
            return this
        }

        /**
         * Adds [ByteArray] value.
         */
        operator fun plus(bytes: ByteArray): Hasher {
            blake2b.update(bytes, 0, bytes.size)
            return this
        }

        internal fun hash(): Hash {
            val bytes = ByteArray(Hash.SIZE)
            blake2b.digest(bytes, 0)
            return Hash(bytes)
        }
    }

    override fun invoke(bytes: ByteArray): ByteArray = hash(bytes).bytes
}
