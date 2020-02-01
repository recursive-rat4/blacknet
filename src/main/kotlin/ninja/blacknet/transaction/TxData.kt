/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.json.JsonElement
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BinaryDecoder

interface TxData {
    fun getType(): TxType
    fun serialize(): ByteArray
    fun toJson(): JsonElement
    fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status

    fun process(tx: Transaction, hash: Hash, ledger: Ledger): Status {
        val account = ledger.get(tx.from)
        if (account == null) {
            return Invalid("Sender account not found")
        }
        if (tx.seq < account.seq) {
            return AlreadyHave("sequence ${tx.seq} expected ${account.seq}")
        } else if (tx.seq > account.seq) {
            return InFuture("sequence ${tx.seq} expected ${account.seq}")
        }
        val status = account.credit(tx.fee)
        if (status != Accepted) {
            return notAccepted("Transaction fee", status)
        }
        account.seq += 1
        ledger.set(tx.from, account)
        return processImpl(tx, hash, 0, ledger)
    }

    companion object {
        fun deserialize(type: Byte, bytes: ByteArray): TxData {
            val serializer = TxType.getSerializer(type)
            return BinaryDecoder.fromBytes(bytes).decode(serializer)
        }
    }
}
