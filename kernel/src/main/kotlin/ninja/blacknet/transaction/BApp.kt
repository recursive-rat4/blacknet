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
import ninja.blacknet.contract.BAppId
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.Ledger
import ninja.blacknet.core.Status
import ninja.blacknet.core.Transaction
import ninja.blacknet.serialization.ByteArraySerializer

/**
 * 黑網區塊鏈應用程序交易
 */
@Serializable
class BApp(
    val id: BAppId,
    @Serializable(with = ByteArraySerializer::class)
    val data: ByteArray
) : TxData {
    override fun processLedgerImpl(tx: Transaction, hash: ByteArray, dataIndex: Int, ledger: Ledger): Status {
        return Accepted
    }

    operator fun component1() = id
    operator fun component2() = data
}
