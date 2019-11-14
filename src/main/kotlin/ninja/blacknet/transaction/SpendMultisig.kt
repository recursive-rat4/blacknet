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
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.json
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.sumByLong

@Serializable
class SpendMultisig(
        val id: Hash,
        val amounts: ArrayList<Long>,
        val signatures: ArrayList<Pair<Byte, Signature>>
) : TxData {
    override fun getType() = TxType.SpendMultisig
    override fun involves(publicKey: PublicKey) = LedgerDB.getMultisig(id)!!.involves(publicKey)
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    fun sign(i: Int, privateKey: PrivateKey): Boolean {
        val signature = Ed25519.sign(hash(), privateKey)
        signatures.add(Pair(i.toByte(), signature))
        return true
    }

    fun verifySignatures(multisig: Multisig): Boolean {
        val multisigHash = hash()
        for (i in signatures) {
            if (signatures.count { it.first == i.first } != 1)
                return false
            val publicKey = multisig.deposits.getOrNull(i.first.toInt())?.first ?: return false
            if (!Ed25519.verify(i.second, multisigHash, publicKey))
                return false
        }
        return true
    }

    private fun hash(): Hash {
        val copy = SpendMultisig(id, amounts, ArrayList())
        val bytes = copy.serialize()
        return Blake2b.hash(bytes)
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger): Status {
        val multisig = ledger.getMultisig(id)
        if (multisig == null) {
            return Invalid("Multisig not found")
        }
        if (amounts.size != multisig.deposits.size) {
            return Invalid("Invalid number of amounts")
        }
        val amount = try {
            amounts.sumByLong()
        } catch (e: ArithmeticException) {
            return Invalid("Invalid total amount: ${e.message}")
        }
        if (amount != multisig.amount()) {
            return Invalid("Invalid total amount")
        }
        if (multisig.deposits.find { it.first == tx.from } == null) {
            return Invalid("Invalid sender")
        }
        if (signatures.size + 1 < multisig.n) {
            return Invalid("Invalid number of signatures")
        }
        if (!verifySignatures(multisig)) {
            return Invalid("Invalid signature")
        }

        val height = ledger.height()

        for (i in multisig.deposits.indices) {
            if (amounts[i] < 0) {
                return Invalid("Negative amount")
            } else if (amounts[i] != 0L) {
                val publicKey = multisig.deposits[i].first
                val toAccount = ledger.getOrCreate(publicKey)
                toAccount.debit(height, amounts[i])
                ledger.set(publicKey, toAccount)
            }
        }

        ledger.removeMultisig(id)
        return Accepted
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val id: String,
            val amounts: JsonArray,
            val signatures: JsonArray
    ) {
        constructor(data: SpendMultisig) : this(
                data.id.toString(),
                JsonArray(data.amounts.map { amount ->
                    JsonPrimitive(amount.toString())
                }),
                JsonArray(data.signatures.map { (i, signature) ->
                    json {
                        "i" to i.toUByte().toInt()
                        "signature" to signature.toString()
                    }
                })
        )
    }
}
