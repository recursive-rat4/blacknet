/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import com.google.common.collect.Maps.newHashMapWithExpectedSize
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.crypto.Blake2b.buildHash
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.LongSerializer
import ninja.blacknet.util.sumByLong

/**
 * 花費合約
 */
@Serializable
class SpendMultisig(
        val id: MultiSignatureLockContractId,
        @Serializable(with = AmountsSerializer::class)
        val amounts: ArrayList<Long>,
        val signatures: ArrayList<SignatureElement>
) : TxData {
    @Serializable
    class SignatureElement(
            val index: Byte,
            val signature: Signature
    ) {
        operator fun component1() = index
        operator fun component2() = signature
    }

    fun sign(i: Int, privateKey: PrivateKey): Boolean {
        val signature = Ed25519.sign(hash(), privateKey)
        signatures.add(SignatureElement(i.toByte(), signature))
        return true
    }

    private fun verifySignatures(multisig: Multisig, sender: PublicKey): Status {
        val multisigHash = hash()
        val unsigned = newHashMapWithExpectedSize<Byte, PublicKey>(multisig.deposits.size)
        for (i in 0 until multisig.deposits.size) {
            unsigned.put(i.toByte(), multisig.deposits[i].first)
        }

        for ((i, signature) in signatures) {
            val publicKey = unsigned.remove(i)
            if (publicKey == null)
                return Invalid("Invalid or twice signed i $i")
            if (!Ed25519.verify(signature, multisigHash, publicKey))
                return Invalid("Invalid signature i $i")
        }

        return if (unsigned.containsValue(sender))
            Accepted
        else
            Invalid("Invalid sender")
    }

    private fun hash(): Hash {
        val copy = SpendMultisig(id, amounts, ArrayList())
        val bytes = BinaryEncoder.toBytes(serializer(), copy)
        return buildHash { encodeByteArray(bytes) }
    }

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
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
        if (signatures.size + 1 < multisig.n) {
            return Invalid("Invalid number of signatures")
        }
        val status = verifySignatures(multisig, tx.from)
        if (status != Accepted) {
            return status
        }

        val height = ledger.height()

        for (index in 0 until multisig.deposits.size) {
            if (amounts[index] < 0) {
                return Invalid("Negative amount index $index")
            } else if (amounts[index] != 0L) {
                val publicKey = multisig.deposits[index].first
                val toAccount = ledger.getOrCreate(publicKey)
                toAccount.debit(height, amounts[index])
                ledger.set(publicKey, toAccount)
            }
        }

        ledger.removeMultisig(id)
        return Accepted
    }

    fun involves(ids: Set<MultiSignatureLockContractId>) = ids.contains(id)

    private object AmountsSerializer : KSerializer<List<Long>> by ListSerializer(LongSerializer)
}
