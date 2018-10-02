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
import ninja.blacknet.serialization.BlacknetInput

private val logger = KotlinLogging.logger {}

interface Ledger {
    fun addSupply(amount: Long)
    fun checkFee(size: Int, amount: Long): Boolean
    fun checkSequence(key: PublicKey, seq: Int): Boolean
    fun height(): Int
    fun get(key: PublicKey): AccountState?
    fun set(key: PublicKey, state: AccountState)

    fun processTransaction(hash: Hash, bytes: ByteArray): Boolean {
        val tx = Transaction.deserialize(bytes)
        if (tx == null) {
            logger.info("deserialization failed")
            return false
        }
        return processTransaction(tx, hash, bytes.size)
    }

    fun processTransaction(tx: Transaction, hash: Hash, size: Int): Boolean {
        if (!tx.verifySignature(hash)) {
            logger.info("invalid signature")
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
        val data = BlacknetInput.fromBytes(tx.data.array).deserialize(serializer)
        if (data == null) {
            logger.info("deserialization of tx data failed")
            return false
        }
        return data.process(tx, this)
    }
}