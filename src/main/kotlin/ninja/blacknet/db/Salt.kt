/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.crypto.PublicKeySerializer
import ninja.blacknet.crypto.nextBytes
import java.security.SecureRandom

private val logger = KotlinLogging.logger {}

object Salt {
    private val OLD_VERSION_KEY = "ledgerversion".toByteArray()
    private val SALT_KEY = DBKey(11, 0)
    private const val SALT_SIZE_BYTES = 16
    val salt: ByteArray

    init {
        if (System.getProperty("org.gradle.test.worker") != null) {
            logger.info("測試避免數據庫")
            salt = ByteArray(SALT_SIZE_BYTES) { it.toByte() }
        } else {
            val saltBytes = LevelDB.get(SALT_KEY)
            salt = if (saltBytes != null && saltBytes.size == SALT_SIZE_BYTES) {
                saltBytes
            } else {
                if (LevelDB.get(OLD_VERSION_KEY) != null)
                    convertor()

                val bytes = SecureRandom().nextBytes(SALT_SIZE_BYTES)

                val batch = LevelDB.createWriteBatch()
                batch.put(SALT_KEY, bytes)
                batch.write()

                bytes
            }
        }
    }

    private fun convertor() {
        logger.info("Converting database; this may take a while...")
        val iterator = LevelDB.iterator()
        iterator.seekToFirst()
        while (iterator.hasNext()) {
            val entry = iterator.next()
            var key: ByteArray?
            key = Pair("account", PublicKeySerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(1, PublicKeySerializer.SIZE_BYTES)); continue }
            key = Pair("chain", HashSerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(2, HashSerializer.SIZE_BYTES)); continue }
            key = Pair("htlc", HashSerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(3, HashSerializer.SIZE_BYTES)); continue }
            key = Pair("multisig", HashSerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(4, HashSerializer.SIZE_BYTES)); continue }
            key = Pair("undo", HashSerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(5, HashSerializer.SIZE_BYTES)); continue }
            key = Pair("ledgersizes", 0) - entry; if (key != null) { nibble(entry, key, DBKey(6, 0)); continue }
            key = Pair("ledgersnapshot", Int.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(7, Int.SIZE_BYTES)); continue }
            key = Pair("ledgersnapshotheights", 0) - entry; if (key != null) { nibble(entry, key, DBKey(8, 0)); continue }
            key = Pair("ledgerstate", 0) - entry; if (key != null) { nibble(entry, key, DBKey(9, 0)); continue }
            key = Pair("ledgerversion", 0) - entry; if (key != null) { continue; }
            key = Pair("walletkeys", 0) - entry; if (key != null) { nibble(entry, key, DBKey(64, 0)); continue }
            key = Pair("tx", HashSerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(65, HashSerializer.SIZE_BYTES)); continue }
            key = Pair("walletversion", 0) - entry; if (key != null) { nibble(entry, key, DBKey(66, 0)); continue }
            key = Pair("wallet", PublicKeySerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(67, PublicKeySerializer.SIZE_BYTES)); continue }
            key = Pair("peerdb", 0) - entry; if (key != null) { nibble(entry, key, DBKey(0x80.toByte(), 0)); continue }
            key = Pair("peerversion", 0) - entry; if (key != null) { nibble(entry, key, DBKey(0x81.toByte(), 0)); continue }
            key = Pair("block", HashSerializer.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(0xC0.toByte(), HashSerializer.SIZE_BYTES)); continue }
            logger.debug { "Unknown key ${Base16.encode(entry.key)}" }
        }
        iterator.close()
        val batch = LevelDB.createWriteBatch()
        val value = LevelDB.get(OLD_VERSION_KEY)!!
        batch.put(DBKey(10, 0), value)
        batch.delete(OLD_VERSION_KEY)
        batch.write()
    }

    private fun nibble(entry: Map.Entry<ByteArray, ByteArray>, key: ByteArray, dbKey: DBKey) {
        val batch = LevelDB.createWriteBatch()
        batch.put(dbKey, key, entry.value)
        batch.delete(entry.key)
        batch.write()
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
