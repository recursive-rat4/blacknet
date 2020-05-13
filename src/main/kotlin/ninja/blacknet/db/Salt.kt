/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import mu.KotlinLogging
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.crypto.nextBytes
import java.security.SecureRandom

private val logger = KotlinLogging.logger {}

object Salt {
    private val OLD_VERSION_KEY = "ledgerversion".toByteArray()
    private val SALT_KEY = DBKey(11, 0)

    val salt: ByteArray

    init {
        val saltBytes = LevelDB.get(SALT_KEY)
        salt = if (saltBytes != null && saltBytes.size == 16) {
            saltBytes
        } else {
            if (LevelDB.get(OLD_VERSION_KEY) != null)
                convertor()

            val bytes = SecureRandom().nextBytes(16)

            val batch = LevelDB.createWriteBatch()
            batch.put(SALT_KEY, bytes)
            batch.write()

            bytes
        }
    }

    private fun convertor() {
        logger.info("Convertoring the database...")
        val iterator = LevelDB.iterator()
        iterator.seekToFirst()
        while (iterator.hasNext()) {
            val entry = iterator.next()
            var key: ByteArray?
            key = Pair("account", PublicKey.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(1, PublicKey.SIZE_BYTES)); continue }
            key = Pair("chain", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(2, Hash.SIZE_BYTES)); continue }
            key = Pair("htlc", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(3, Hash.SIZE_BYTES)); continue }
            key = Pair("multisig", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(4, Hash.SIZE_BYTES)); continue }
            key = Pair("undo", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(5, Hash.SIZE_BYTES)); continue }
            key = Pair("ledgersizes", 0) - entry; if (key != null) { nibble(entry, key, DBKey(6, 0)); continue }
            key = Pair("ledgersnapshot", Int.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(7, Int.SIZE_BYTES)); continue }
            key = Pair("ledgersnapshotheights", 0) - entry; if (key != null) { nibble(entry, key, DBKey(8, 0)); continue }
            key = Pair("ledgerstate", 0) - entry; if (key != null) { nibble(entry, key, DBKey(9, 0)); continue }
            key = Pair("ledgerversion", 0) - entry; if (key != null) { continue; }
            key = Pair("walletkeys", 0) - entry; if (key != null) { nibble(entry, key, DBKey(64, 0)); continue }
            key = Pair("tx", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(65, Hash.SIZE_BYTES)); continue }
            key = Pair("walletversion", 0) - entry; if (key != null) { nibble(entry, key, DBKey(66, 0)); continue }
            key = Pair("wallet", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(67, Hash.SIZE_BYTES)); continue }
            key = Pair("peerdb", 0) - entry; if (key != null) { nibble(entry, key, DBKey(0x80.toByte(), 0)); continue }
            key = Pair("peerversion", 0) - entry; if (key != null) { nibble(entry, key, DBKey(0x81.toByte(), 0)); continue }
            key = Pair("block", Hash.SIZE_BYTES) - entry; if (key != null) { nibble(entry, key, DBKey(0xC0.toByte(), Hash.SIZE_BYTES)); continue }
            logger.debug { "Unknown key ${entry.key.toHex()}" }
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
