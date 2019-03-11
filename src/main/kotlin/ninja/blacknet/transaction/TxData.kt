/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import mu.KotlinLogging
import ninja.blacknet.core.Ledger
import ninja.blacknet.core.Transaction
import ninja.blacknet.core.UndoBlock
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BlacknetDecoder

private val logger = KotlinLogging.logger {}

interface TxData {
    fun serialize(): ByteArray
    fun getType(): Byte
    suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBlock): Boolean

    suspend fun process(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBlock): Boolean {
        val account = ledger.get(tx.from)
        if (account == null) {
            logger.info("account not found")
            return false
        }
        if (tx.seq != account.seq) {
            logger.info("invalid sequence number")
            return false
        }
        undo.add(tx.from, account.copy())
        if (!account.credit(tx.fee)) {
            logger.info("insufficient funds for tx fee")
            return false
        }
        account.prune(ledger.height())
        account.seq++
        ledger.set(tx.from, account)
        return processImpl(tx, hash, ledger, undo)
    }

    companion object {
        fun deserialize(type: Byte, bytes: ByteArray): TxData? {
            val serializer = TxType.getSerializer(type)
            if (serializer == null) {
                logger.info("unknown transaction type:$type")
                return null
            }
            return BlacknetDecoder.fromBytes(bytes).decode(serializer)
        }
    }
}
