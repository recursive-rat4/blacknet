/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.json.JsonElement
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder

private val logger = KotlinLogging.logger {}

interface TxData {
    fun getType(): TxType
    fun involves(publicKey: PublicKey): Boolean
    fun serialize(): ByteArray
    fun toJson(): JsonElement
    suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger): Status

    suspend fun process(tx: Transaction, hash: Hash, ledger: Ledger): Status {
        val account = ledger.get(tx.from)
        if (account == null) {
            return Invalid("Sender account not found")
        }
        if (tx.seq < account.seq) {
            logger.debug { "already have seq ${tx.seq}" }
            return AlreadyHave
        } else if (tx.seq > account.seq) {
            logger.debug { "in future seq ${tx.seq}" }
            return InFuture
        }
        val status = account.credit(tx.fee)
        if (status != Accepted) {
            return if (status is Invalid)
                Invalid("${status.reason} for tx fee")
            else
                status
        }
        account.seq += 1
        ledger.set(tx.from, account)
        return processImpl(tx, hash, ledger)
    }

    companion object {
        fun deserialize(type: Byte, bytes: ByteArray): TxData {
            val serializer = TxType.getSerializer(type)
            return BinaryDecoder.fromBytes(bytes).decode(serializer)
        }
    }
}
