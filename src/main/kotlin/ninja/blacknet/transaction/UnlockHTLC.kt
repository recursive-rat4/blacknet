/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class UnlockHTLC(
        val id: Hash,
        val preimage: SerializableByteArray,
        var signatureB: Signature
) : TxData {
    override fun getType() = TxType.UnlockHTLC
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(serializer(), this)

    fun sign(privateKey: PrivateKey) {
        val bytes = serialize()
        signatureB = Ed25519.sign(hash(bytes), privateKey)
        System.arraycopy(signatureB.bytes, 0, bytes, 0, Signature.SIZE)
    }

    fun verifySignature(publicKey: PublicKey): Boolean {
        val bytes = serialize()
        return Ed25519.verify(signatureB, hash(bytes), publicKey)
    }

    private fun hash(bytes: ByteArray): Hash {
        return Blake2b.hash(bytes, 0, bytes.size - Signature.SIZE)
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            return Invalid("HTLC not found")
        }
        if (!htlc.verifyHashLock(preimage)) {
            return Invalid("Invalid hashlock")
        }
        if (!verifySignature(htlc.to)) {
            return Invalid("Invalid signature")
        }

        val toAccount = ledger.getOrCreate(htlc.to)
        toAccount.debit(ledger.height(), htlc.amount)
        ledger.set(htlc.to, toAccount)
        ledger.removeHTLC(id)
        return Accepted
    }

    fun involves(ids: Set<Hash>) = ids.contains(id)

    companion object {
        fun deserialize(bytes: ByteArray): UnlockHTLC = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }
}
