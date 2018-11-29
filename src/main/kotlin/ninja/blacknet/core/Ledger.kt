/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import mu.KotlinLogging
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.BlockDB
import ninja.blacknet.serialization.BlacknetDecoder
import ninja.blacknet.transaction.TxType

private val logger = KotlinLogging.logger {}

interface Ledger {
    fun addSupply(amount: Long)
    fun addUndo(hash: Hash, undo: UndoBlock)
    fun checkFee(size: Int, amount: Long): Boolean
    fun blockTime(): Long
    fun height(): Int
    suspend fun checkSequence(key: PublicKey, seq: Int): Boolean
    suspend fun get(key: PublicKey): AccountState?
    suspend fun set(key: PublicKey, state: AccountState)
    fun addHTLC(id: Hash, htlc: HTLC)
    fun getHTLC(id: Hash): HTLC?
    fun removeHTLC(id: Hash)
    fun addMultisig(id: Hash, multisig: Multisig)
    fun getMultisig(id: Hash): Multisig?
    fun removeMultisig(id: Hash)

    suspend fun getOrCreate(key: PublicKey) = get(key) ?: AccountState.create()

    suspend fun processTransaction(hash: Hash, bytes: ByteArray, undo: UndoBlock): Boolean {
        val tx = Transaction.deserialize(bytes)
        if (tx == null) {
            logger.info("deserialization failed")
            return false
        }
        return processTransaction(tx, hash, bytes.size, undo)
    }

    suspend fun processTransaction(tx: Transaction, hash: Hash, size: Int, undo: UndoBlock): Boolean {
        if (!tx.verifySignature(hash)) {
            logger.info("invalid signature")
            return false
        }
        if (tx.blochHash != Hash.ZERO && !BlockDB.contains(tx.blochHash)) {
            logger.info("not valid on this chain")
            return false
        }
        if (!checkFee(size, tx.fee)) {
            logger.info("too low fee ${tx.fee}")
            return false
        }
        val serializer = TxType.getSerializer(tx.type)
        if (serializer == null) {
            logger.info("unknown transaction type ${tx.type}")
            return false
        }
        val data = BlacknetDecoder.fromBytes(tx.data.array).decode(serializer)
        if (data == null) {
            logger.info("deserialization of tx data failed")
            return false
        }
        return data.process(tx, hash, this, undo)
    }
}
