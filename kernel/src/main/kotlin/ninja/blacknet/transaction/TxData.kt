/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash

interface TxData {
    fun processCoinImpl(tx: Transaction, hash: Hash, dataIndex: Int, coinTx: CoinTx): Status

    fun processCoin(tx: Transaction, hash: Hash, coinTx: CoinTx): Status {
        val account = coinTx.getAccount(tx.from)
        if (account == null) {
            return Invalid("Sender account not found")
        }
        if (tx.seq != account.seq) {
            if (tx.seq.toUInt() < account.seq.toUInt()) {
                return AlreadyHave("sequence ${tx.seq} expected ${account.seq}")
            } else {
                require(tx.seq.toUInt() > account.seq.toUInt())
                return InFuture("sequence ${tx.seq} expected ${account.seq}")
            }
        }
        val status = account.credit(tx.fee)
        if (status != Accepted) {
            return notAccepted("Transaction fee", status)
        }
        account.seq += 1
        coinTx.setAccount(tx.from, account)
        return processCoinImpl(tx, hash, 0, coinTx)
    }
}
