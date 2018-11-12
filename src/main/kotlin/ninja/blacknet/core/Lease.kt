/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.readBytes
import kotlinx.serialization.Serializable
import kotlinx.serialization.encode
import mu.KotlinLogging
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
        return TxType.Lease.ordinal.toByte()
    }

    override suspend fun processImpl(tx: Transaction, account: AccountState, ledger: Ledger, undo: UndoList): Boolean {
        if (amount < PoS.MIN_LEASE) {
            logger.info("$amount less than minimal ${PoS.MIN_LEASE}")
            return false
        }
        if (!account.credit(amount))
            return false
        val toAccount = ledger.get(to) ?: AccountState.create()
        undo.add(Pair(to, toAccount.copy()))
        toAccount.leases.add(AccountState.Input(ledger.height(), amount))
        ledger.set(to, toAccount)
        return true
    }
}