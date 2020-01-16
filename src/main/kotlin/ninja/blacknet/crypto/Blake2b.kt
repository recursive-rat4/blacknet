/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import com.rfksystems.blake2b.Blake2b
import ninja.blacknet.SystemService

/**
 * BLAKE2b-256 hash function.
 */
@SystemService
object Blake2b : (ByteArray) -> ByteArray {
    const val DIGEST_SIZE = 256
    const val HASH_SIZE = DIGEST_SIZE / 8

    /**
     * Returns a hash with given [digestSize].
     */
    internal fun hash(digestSize: Int, message: ByteArray): ByteArray {
        val bytes = ByteArray(digestSize / 8)
        val b = Blake2b(digestSize)
        b.update(message, 0, message.size)
        b.digest(bytes, 0)
        return bytes
    }

    /**
     * Builds [Hash] with given [init] builder.
     */
    fun hasher(init: Hasher.() -> Unit): Hash {
        val hasher = Hasher()
        hasher.init()
        return hasher.result()
    }

    /**
     * DSL builder for a [Hash].
     */
    class Hasher(private val blake2b: Blake2b) {
        internal constructor() : this(Blake2b(DIGEST_SIZE))

        /**
         * Adds [Byte] value.
         */
        fun x(byte: Byte) {
            blake2b.update(byte)
        }

        /**
         * Adds [Short] value in big-endian byte order.
         */
        fun x(short: Short) {
            blake2b.update((short.toInt() shr 8).toByte())
            blake2b.update(short.toByte())
        }

        /**
         * Adds [Int] value in big-endian byte order.
         */
        fun x(int: Int) {
            blake2b.update((int shr 24).toByte())
            blake2b.update((int shr 16).toByte())
            blake2b.update((int shr 8).toByte())
            blake2b.update(int.toByte())
        }

        /**
         * Adds [Long] value in big-endian byte order.
         */
        fun x(long: Long) {
            blake2b.update((long shr 56).toByte())
            blake2b.update((long shr 48).toByte())
            blake2b.update((long shr 40).toByte())
            blake2b.update((long shr 32).toByte())
            blake2b.update((long shr 24).toByte())
            blake2b.update((long shr 16).toByte())
            blake2b.update((long shr 8).toByte())
            blake2b.update(long.toByte())
        }

        /**
         * Adds [String] value using the UTF-8 charset.
         */
        fun x(string: String) {
            val bytes = string.toByteArray(Charsets.UTF_8)
            blake2b.update(bytes, 0, bytes.size)
        }

        /**
         * Adds [ByteArray] value.
         */
        fun x(bytes: ByteArray) {
            blake2b.update(bytes, 0, bytes.size)
        }

        /**
         * Adds a block of bytes.
         *
         * @param bytes the [ByteArray] containing the data
         * @param offset the offset of the data
         * @param length the length of the data
         */
        fun x(bytes: ByteArray, offset: Int, length: Int) {
            blake2b.update(bytes, offset, length)
        }

        /**
         * Adds [Hash] value.
         */
        fun x(hash: Hash) {
            blake2b.update(hash.bytes, 0, Hash.SIZE)
        }

        /**
         * Adds [PublicKey] value.
         */
        fun x(publicKey: PublicKey) {
            blake2b.update(publicKey.bytes, 0, PublicKey.SIZE)
        }

        internal fun result(): Hash {
            val bytes = ByteArray(Hash.SIZE)
            blake2b.digest(bytes, 0)
            return Hash(bytes)
        }
    }

    override fun invoke(bytes: ByteArray): ByteArray = hasher { x(bytes) }.bytes
}
