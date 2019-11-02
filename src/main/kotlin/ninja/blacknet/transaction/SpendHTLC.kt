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
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.db.WalletDB
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.toHex

@Serializable
class SpendHTLC(
        val id: Hash,
        val amountA: Long,
        val amountB: Long,
        var signatureB: Signature
) : TxData {
    override fun getType() = TxType.SpendHTLC
    override fun involves(publicKey: PublicKey) = WalletDB.involves(id, publicKey)
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

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

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger): Status {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            return Invalid("HTLC not found")
        }
        if (amountA < 0 || amountB < 0) {
            return Invalid("Negative amount")
        }
        val amount = try {
            Math.addExact(amountA, amountB)
        } catch (e: ArithmeticException) {
            return Invalid("Invalid amount: ${e.message}")
        }
        if (amount != htlc.amount) {
            return Invalid("Invalid amount")
        }
        if (tx.from != htlc.from) {
            return Invalid("Invalid sender")
        }
        if (!verifySignature(htlc.to)) {
            return Invalid("Invalid signature")
        }

        val height = ledger.height()
        val toAccount = ledger.getOrCreate(htlc.to)
        toAccount.debit(height, amountB)
        ledger.set(htlc.to, toAccount)
        val account = ledger.get(tx.from)!!
        account.debit(height, amountA)
        ledger.set(tx.from, account)
        ledger.removeHTLC(id)
        return Accepted
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val id: String,
            val amountA: String,
            val amountB: String,
            val signatureB: String
    ) {
        constructor(data: SpendHTLC) : this(
                data.id.bytes.toHex(),
                data.amountA.toString(),
                data.amountB.toString(),
                data.signatureB.bytes.toHex()
        )
    }
}
