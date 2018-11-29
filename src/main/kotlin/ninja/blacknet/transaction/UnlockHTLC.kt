/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.io.core.readBytes
import kotlinx.serialization.Serializable
import kotlinx.serialization.encode
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

private val logger = KotlinLogging.logger {}

@Serializable
class UnlockHTLC(
        val id: Hash,
        val preimage: SerializableByteArray,
        var signatureB: Signature
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.UnlockHTLC.ordinal.toByte()
    }

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

    override suspend fun processImpl(tx: Transaction, hash: Hash, account: AccountState, ledger: Ledger, undo: UndoBlock): Boolean {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            logger.info("htlc not found")
            return false
        }
        if (!htlc.verifyHashLock(preimage)) {
            logger.info("invalid hashlock")
            return false
        }
        if (!verifySignature(htlc.to)) {
            logger.info("invalid signature")
            return false
        }

        val toAccount = ledger.getOrCreate(htlc.to)
        undo.add(htlc.to, toAccount.copy())
        undo.addHTLC(id, htlc)

        toAccount.debit(ledger.height(), htlc.amount)
        ledger.removeHTLC(id)
        return true
    }
}
