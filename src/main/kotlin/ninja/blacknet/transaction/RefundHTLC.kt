/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.WalletDB
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json

@Serializable
class RefundHTLC(
        val id: Hash
) : TxData {
    override fun getType() = TxType.RefundHTLC
    override fun involves(publicKey: PublicKey) = WalletDB.involves(id, publicKey)
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(serializer(), this)

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger): Status {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            return Invalid("HTLC not found")
        }
        if (tx.from != htlc.from) {
            return Invalid("Invalid sender")
        }
        if (!htlc.verifyTimeLock(ledger)) {
            return Invalid("Invalid timelock")
        }

        val account = ledger.get(tx.from)!!
        account.debit(ledger.height(), htlc.amount)
        ledger.set(tx.from, account)
        ledger.removeHTLC(id)
        return Accepted
    }
}
