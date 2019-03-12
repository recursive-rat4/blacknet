/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BlacknetEncoder

private val logger = KotlinLogging.logger {}

@Serializable
class SpendHTLC(
        val id: Hash,
        val amountA: Long,
        val amountB: Long,
        var signatureB: Signature
) : TxData {
    override fun serialize() = BlacknetEncoder.toBytes(serializer(), this)

    override fun getType() = TxType.SpendHTLC

    fun sign(privateKey: PrivateKey) {
        val bytes = serialize()
        signatureB = Ed25519.sign(hash(bytes), privateKey)
        System.arraycopy(signatureB.bytes.array, 0, bytes, 0, Signature.SIZE)
    }

    fun verifySignature(publicKey: PublicKey): Boolean {
        val bytes = serialize()
        return Ed25519.verify(signatureB, hash(bytes), publicKey)
    }

    private fun hash(bytes: ByteArray): Hash {
        return Blake2b.hash(bytes, 0, bytes.size - Signature.SIZE)
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBlock): Boolean {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            logger.info("htlc not found")
            return false
        }
        if (amountA < 0 || amountB < 0) {
            logger.info("negative amount")
            return false
        }
        if (amountA + amountB != htlc.amount) {
            logger.info("invalid amount")
            return false
        }
        if (tx.from != htlc.from) {
            logger.info("invalid sender")
            return false
        }
        if (!verifySignature(htlc.to)) {
            logger.info("invalid signature")
            return false
        }

        val height = ledger.height()
        val toAccount = ledger.getOrCreate(htlc.to)
        undo.add(htlc.to, toAccount.copy())
        undo.addHTLC(id, htlc)

        toAccount.debit(height, amountB)
        ledger.set(htlc.to, toAccount)
        val account = ledger.get(tx.from)!!
        account.debit(height, amountA)
        ledger.set(tx.from, account)
        ledger.removeHTLC(id)
        return true
    }
}
