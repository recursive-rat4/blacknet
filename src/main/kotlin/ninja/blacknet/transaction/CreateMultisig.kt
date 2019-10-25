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
import kotlinx.serialization.json.json
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.sumByLong

@Serializable
class CreateMultisig(
        val n: Byte,
        val deposits: ArrayList<Pair<PublicKey, Long>>,
        val signatures: ArrayList<Pair<Byte, Signature>>
) : TxData {
    override fun getType() = TxType.CreateMultisig
    override fun involves(publicKey: PublicKey) = deposits.find { it.first == publicKey } != null
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    fun sign(from: PublicKey, seq: Int, privateKey: PrivateKey): Boolean {
        val publicKey = privateKey.toPublicKey()
        val i = deposits.indexOfFirst { it.first == publicKey }
        if (i == -1) return false
        val signature = Ed25519.sign(hash(from, seq), privateKey)
        signatures.add(Pair(i.toByte(), signature))
        return true
    }

    fun verifySignature(signature: Signature, hash: Hash, publicKey: PublicKey): Boolean {
        return Ed25519.verify(signature, hash, publicKey)
    }

    private fun hash(from: PublicKey, seq: Int): Hash {
        val copy = CreateMultisig(n, deposits, ArrayList())
        val bytes = copy.serialize()
        return (Blake2b.Hasher() + from.bytes + seq + bytes).hash()
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger): Status {
        if (n < 0 || n > deposits.size) {
            return Invalid("Invalid n")
        }
        if (deposits.size > 20) {
            return Invalid("Too many deposits")
        }
        if (signatures.size > deposits.size) {
            return Invalid("Too many signatures")
        }
        val total = try {
            deposits.sumByLong { it.second }
        } catch (e: ArithmeticException) {
            return Invalid("Invalid total amount: ${e.message}")
        }
        if (total <= 0) {
            return Invalid("Invalid total amount")
        }

        val multisigHash = hash(tx.from, tx.seq)
        for (i in deposits.indices) {
            if (deposits[i].second != 0L) {
                val signature = signatures.find { it.first == i.toByte() }?.second
                if (signature == null) {
                    return Invalid("Unsigned deposit")
                }
                if (!verifySignature(signature, multisigHash, deposits[i].first)) {
                    return Invalid("Invalid signature")
                }
                val depositAccount = ledger.get(deposits[i].first)
                if (depositAccount == null) {
                    return Invalid("Account not found")
                }
                val status = depositAccount.credit(deposits[i].second)
                if (status != Accepted) {
                    return status
                }
                ledger.set(deposits[i].first, depositAccount)
            }
        }

        val multisig = Multisig(n, deposits)
        ledger.addMultisig(hash, multisig)
        return Accepted
    }

    companion object {
        fun deserialize(bytes: ByteArray): CreateMultisig = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val n: Int,
            val deposits: JsonArray,
            val signatures: JsonArray
    ) {
        constructor(data: CreateMultisig) : this(
                data.n.toUByte().toInt(),
                JsonArray(data.deposits.map { (publicKey, amount) ->
                    json {
                        "from" to Address.encode(publicKey)
                        "amount" to amount.toString()
                    }
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
