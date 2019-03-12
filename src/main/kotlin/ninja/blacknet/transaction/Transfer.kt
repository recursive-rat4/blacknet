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
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.Message
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetEncoder

@Serializable
class Transfer(
        val amount: Long,
        val to: PublicKey,
        val message: Message
) : TxData {
    override fun serialize() = BlacknetEncoder.toBytes(serializer(), this)

    override fun getType() = TxType.Transfer

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBlock): Boolean {
        val account = ledger.get(tx.from)!!
        if (!account.credit(amount))
            return false
        ledger.set(tx.from, account)
        val toAccount = ledger.getOrCreate(to)
        undo.add(to, toAccount.copy())
        toAccount.debit(ledger.height(), amount)
        ledger.set(to, toAccount)
        return true
    }
}