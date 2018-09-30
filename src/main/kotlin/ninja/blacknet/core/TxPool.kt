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
import ninja.blacknet.db.Ledger

private val logger = KotlinLogging.logger {}

object TxPool : MemPool() {
    private fun checkSequence(publicKey: PublicKey, seq: Int): Boolean {
        return Ledger.checkSequence(publicKey, seq)
    }

    override fun processImpl(hash: Hash, bytes: ByteArray): Boolean {
        val tx = Transaction.deserialize(bytes)
        if (tx == null) {
            logger.info("deserialization failed")
            return false
        }
        if (!checkSequence(tx.from, tx.seq)) {
            logger.info("invalid sequence number")
            return false
        }
        if (!tx.verifySignature(hash)) {
            logger.info("invalid signature")
            return false
        }
        if (tx.fee < 0) {
            logger.info("negative fee")
            return false
        }
        //TODO
        return false
    }
}