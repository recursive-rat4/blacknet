/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.ByteReadPacket
import mu.KotlinLogging

private val logger = KotlinLogging.logger {}

interface TxData {
    fun serialize(): ByteReadPacket
    fun getType(): Byte
    fun processImpl(tx: Transaction, account: AccountState, ledger: Ledger): Boolean

    fun process(tx: Transaction, ledger: Ledger): Boolean {
        val account = ledger.get(tx.from)
        if (account == null) {
            logger.info("account not found")
            return false
        }
        if (tx.seq != account.seq) {
            logger.info("invalid sequence number")
            return false
        }
        if (!account.credit(tx.fee)) {
            logger.info("insufficient funds for tx fee")
            return false
        }
        if (processImpl(tx, account, ledger)) {
            account.prune(ledger.height())
            account.seq++
            ledger.set(tx.from, account)
            return true
        }
        return false
    }
}