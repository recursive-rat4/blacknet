/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.core.*

@Serializable
class Dispel(
) : TxData {
    override fun processLedgerImpl(tx: Transaction, hash: ByteArray, dataIndex: Int, ledger: Ledger): Status {
        return if (tx.fee > 0)
            Accepted
        else
            Invalid("Invalid transaction fee")
    }
}
