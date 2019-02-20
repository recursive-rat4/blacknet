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
import ninja.blacknet.util.sumByLong

private val logger = KotlinLogging.logger {}

@Serializable
class CreateMultisig(
        val n: Byte,
        val deposits: ArrayList<Pair<PublicKey, Long>>,
        val signatures: ArrayList<Pair<Byte, Signature>>
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.CreateMultisig.type
    }

    fun sign(privateKey: PrivateKey): Boolean {
        val publicKey = privateKey.toPublicKey()
        val i = deposits.indexOfFirst { it.first == publicKey }
        if (i == -1) return false
        val signature = Ed25519.sign(hash(), privateKey)
        signatures.add(Pair(i.toByte(), signature))
        return true
    }

    fun verifySignature(signature: Signature, hash: Hash, publicKey: PublicKey): Boolean {
        return Ed25519.verify(signature, hash, publicKey)
    }

    private fun hash(): Hash {
        val copy = CreateMultisig(n, deposits, ArrayList())
        val bytes = copy.serialize()
        return Blake2b.hash(bytes)
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBlock): Boolean {
        if (n < 0 || n > deposits.size) {
            logger.info("invalid n")
            return false
        }
        if (deposits.size > 20) {
            logger.info("too many deposits")
            return false
        }
        if (signatures.size > deposits.size) {
            logger.info("too many signatures")
            return false
        }
        val total = deposits.sumByLong { it.second }
        if (total <= 0) {
            logger.info("invalid total amount")
            return false
        }

        val multisigHash = hash()
        for (i in deposits.indices) {
            if (deposits[i].second != 0L) {
                val signature = signatures.find { it.first == i.toByte() }?.second
                if (signature == null) {
                    logger.info("unsigned deposit")
                    return false
                }
                if (!verifySignature(signature, multisigHash, deposits[i].first)) {
                    logger.info("invalid signature")
                    return false
                }
                val depositAccount = ledger.get(deposits[i].first)
                if (depositAccount == null) {
                    logger.info("account not found")
                    return false
                }
                undo.add(deposits[i].first, depositAccount.copy())
                if (!depositAccount.credit(deposits[i].second))
                    return false
                ledger.set(deposits[i].first, depositAccount)
            }
        }

        undo.addMultisig(hash, null)

        val keys = ArrayList<PublicKey>(deposits.size)
        deposits.mapTo(keys) { it.first }
        val multisig = Multisig(total, n, keys)
        ledger.addMultisig(hash, multisig)
        return true
    }
}
