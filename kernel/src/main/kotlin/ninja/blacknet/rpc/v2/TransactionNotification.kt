/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.db.WalletDB

@Serializable
class TransactionNotification(
        val hash: String,
        val time: Long,
        val size: Int,
        val signature: String,
        val from: String,
        val seq: Int,
        @SerialName("referenceChain")
        val anchor: String,
        val fee: String,
        val type: UByte,
        val data: List<TransactionInfo.DataInfo>
) {
    constructor(tx: Transaction, hash: Hash, time: Long, size: Int, filter: List<WalletDB.TransactionDataType>? = null) : this(
            hash.toString(),
            time,
            size,
            SignatureSerializer.encode(tx.signature),
            Address.encode(tx.from.bytes),
            tx.seq,
            tx.anchor.toString(),
            tx.fee.toString(),
            tx.type,
            TransactionInfo.data(tx.type, tx.data, filter)
    )
}
