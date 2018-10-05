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
import mu.KotlinLogging
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetOutput

private val logger = KotlinLogging.logger {}

@Serializable
class CancelLease(
        val amount: Long,
        val to: PublicKey,
        val height: Int
) : TxData {
    override fun serialize(): ByteArray {
        val out = BlacknetOutput()
        out.write(this)
        return out.build().readBytes()
    }

    override fun getType(): Byte {
        return TxType.CancelLease.ordinal.toByte()
    }

    override fun processImpl(tx: Transaction, account: AccountState, ledger: Ledger): Boolean {
        val toAccount = ledger.get(to)
        if (toAccount == null) {
            logger.info("account not found")
            return false
        }
        if (toAccount.leases.remove(AccountState.Input(height, amount))) {
            account.debit(ledger.height(), amount)
            ledger.set(to, toAccount)
            return true
        }
        logger.info("lease not found")
        return false
    }
}