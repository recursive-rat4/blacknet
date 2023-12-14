/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.ByteArraySerializer

/**
 * 宣稱合約
 */
@Serializable
class ClaimHTLC(
        val id: HashTimeLockContractId,
        @Serializable(with = ByteArraySerializer::class)
        val preimage: ByteArray
) : TxData {
    override fun processLedgerImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            return Invalid("HTLC not found")
        }
        if (tx.from != htlc.to) {
            return Invalid("Invalid sender")
        }
        if (!htlc.hashLock.verify(preimage)) {
            return Invalid("Invalid hash lock")
        }

        val account = ledger.getAccount(tx.from)!!
        account.debit(ledger.height(), htlc.amount)
        ledger.setAccount(tx.from, account)
        ledger.removeHTLC(id)
        return Accepted
    }

    fun involves(ids: Set<HashTimeLockContractId>) = ids.contains(id)
}
