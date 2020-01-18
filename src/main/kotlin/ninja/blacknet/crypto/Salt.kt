/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import com.google.common.primitives.Ints
import mu.KotlinLogging
import ninja.blacknet.SystemService
import ninja.blacknet.db.DBKey
import ninja.blacknet.db.LevelDB
import ninja.blacknet.serialization.toHex
import ninja.blacknet.util.emptyByteArray
import kotlin.random.Random

private val logger = KotlinLogging.logger {}

@SystemService
object Salt {
    private val OLD_VERSION_KEY = "ledgerversion".toByteArray()
    private val SALT_KEY = DBKey(11, 0)
    private val salt: Int

    init {
        val saltBytes = LevelDB.get(SALT_KEY, emptyByteArray())
        salt = if (saltBytes != null) {
            Ints.fromBytes(saltBytes[0], saltBytes[1], saltBytes[2], saltBytes[3])
        } else {
            if (LevelDB.get(OLD_VERSION_KEY) != null)
                nibbler()

            val bytes = Random.nextBytes(Int.SIZE_BYTES + Hash.SIZE)

            val batch = LevelDB.createWriteBatch()
            batch.put(SALT_KEY, emptyByteArray(), bytes)
            batch.write()

            Ints.fromBytes(bytes[0], bytes[1], bytes[2], bytes[3])
        }
    }

    /**
     * Builds a hash code with given [init] builder.
     */
    fun hashCode(init: HashCoder.() -> Unit): Int {
        val coder = HashCoder()
        coder.init()
        return coder.result()
    }

    /**
     * DSL builder for a hash code.
     */
    class HashCoder(private var x: Int) {
        internal constructor() : this(salt)

        /**
         * Adds [Byte] value.
         */
        fun x(byte: Byte) {
            f(byte.toInt())
        }

        /**
         * Adds [Short] value.
         */
        fun x(short: Short) {
            f(short.toInt())
        }

        /**
         * Adds [Int] value.
         */
        fun x(int: Int) {
            f(int)
        }

        /**
         * Adds [Long] value.
         */
        fun x(long: Long) {
            f((long / 4294967296L + long).toInt())
        }

        /**
         * Adds [ByteArray] value.
         */
        fun x(bytes: ByteArray) {
            for (i in 0 until bytes.size)
                f(bytes[i].toInt())
        }

        internal fun result(): Int {
            return x
        }

        private fun f(x: Int) {
            this.x = 31 * this.x + x;
        }
    }

    private fun nibbler() {
        logger.debug { "Nibbling the keys..." }
        val batch = LevelDB.createWriteBatch()
        val iterator = LevelDB.iterator()
        iterator.seekToFirst()
        while (iterator.hasNext()) {
            val entry = iterator.next()
            var key: ByteArray?
            key = Pair("account", PublicKey.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(1, PublicKey.SIZE)); continue }
            key = Pair("chain", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(2, Hash.SIZE)); continue }
            key = Pair("htlc", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(3, Hash.SIZE)); continue }
            key = Pair("multisig", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(4, Hash.SIZE)); continue }
            key = Pair("undo", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(5, Hash.SIZE)); continue }
            key = Pair("ledgersizes", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(6, 0)); continue }
            key = Pair("ledgersnapshot", Int.SIZE_BYTES) - entry; if (key != null) { nibble(batch, entry, key, DBKey(7, Int.SIZE_BYTES)); continue }
            key = Pair("ledgersnapshotheights", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(8, 0)); continue }
            key = Pair("ledgerstate", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(9, 0)); continue }
            key = Pair("ledgerversion", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(10, 0)); continue }
            key = Pair("walletkeys", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(64, 0)); continue }
            key = Pair("tx", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(65, Hash.SIZE)); continue }
            key = Pair("walletversion", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(66, 0)); continue }
            key = Pair("wallet", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(67, Hash.SIZE)); continue }
            key = Pair("peerdb", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(0x80.toByte(), 0)); continue }
            key = Pair("peerversion", 0) - entry; if (key != null) { nibble(batch, entry, key, DBKey(0x81.toByte(), 0)); continue }
            key = Pair("block", Hash.SIZE) - entry; if (key != null) { nibble(batch, entry, key, DBKey(0xC0.toByte(), Hash.SIZE)); continue }
            logger.info("Unknown key ${entry.key.toHex()}")
        }
        iterator.close()
        batch.write()
    }

    private fun nibble(batch: LevelDB.WriteBatch, entry: Map.Entry<ByteArray, ByteArray>, key: ByteArray, dbKey: DBKey) {
        batch.put(dbKey, key, entry.value)
        batch.delete(entry.key)
    }
}

private operator fun Pair<String, Int>.minus(entry: Map.Entry<ByteArray, *>): ByteArray? {
    return if (entry.key.size != first.length + second) {
        null
    } else {
        if (!entry.key.sliceArray(0 until first.length).contentEquals(first.toByteArray(Charsets.US_ASCII))) {
            null
        } else {
            entry.key.sliceArray(first.length until entry.key.size)
        }
    }
}
