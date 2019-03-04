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
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetEncoder

private val logger = KotlinLogging.logger {}

@Serializable
class Lease(
        val amount: Long,
        val to: PublicKey
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.Lease.type
    }

    override suspend fun processImpl(tx: Transaction, hash: Hash, ledger: Ledger, undo: UndoBlock): Boolean {
        if (amount < PoS.MIN_LEASE) {
            logger.info("$amount less than minimal ${PoS.MIN_LEASE}")
            return false
        }
        val account = ledger.get(tx.from)!!
        if (!account.credit(amount))
            return false
        ledger.set(tx.from, account)
        val toAccount = ledger.getOrCreate(to)
        undo.add(to, toAccount.copy())
        toAccount.leases.add(AccountState.LeaseInput(tx.from, ledger.height(), amount))
        ledger.set(to, toAccount)
        return true
    }
}