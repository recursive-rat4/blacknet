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

private val logger = KotlinLogging.logger {}

@Serializable
class SpendMultisig(
        val id: Hash,
        val amounts: ArrayList<Long>,
        val signatures: ArrayList<Pair<Byte, Signature>>
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.SpendMultisig.type
    }

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
            val publicKey = multisig.keys.getOrNull(i.first.toInt()) ?: return false
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

    override suspend fun processImpl(tx: Transaction, hash: Hash, account: AccountState, ledger: Ledger, undo: UndoBlock): Boolean {
        val multisig = ledger.getMultisig(id)
        if (multisig == null) {
            logger.info("multisig not found")
            return false
        }
        if (amounts.size != multisig.keys.size) {
            logger.info("invalid number of amounts")
            return false
        }
        if (amounts.sum() != multisig.amount) {
            logger.info("invalid total amount")
            return false
        }
        if (!multisig.keys.contains(tx.from)) {
            logger.info("invalid sender")
            return false
        }
        if (signatures.size + 1 < multisig.n) {
            logger.info("invalid number of signatures")
            return false
        }
        if (!verifySignatures(multisig)) {
            logger.info("invalid signature")
            return false
        }

        val height = ledger.height()
        undo.addMultisig(id, multisig)

        for (i in multisig.keys.indices) {
            if (amounts[i] < 0) {
                logger.info("negative amount")
                return false
            } else if (amounts[i] != 0L) {
                val toAccount = ledger.getOrCreate(multisig.keys[i])
                undo.add(multisig.keys[i], toAccount.copy())
                toAccount.debit(height, amounts[i])
            }
        }

        ledger.removeMultisig(id)
        return true
    }
}
