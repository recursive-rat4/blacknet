/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.contract.DAppId
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.Ledger
import ninja.blacknet.core.Status
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.Json

/**
 * 分散式應用程序交易包
 */
@Serializable
class Bundle(
        val id: DAppId,
        @Serializable(with = ByteArraySerializer::class)
        val data: ByteArray
) : TxData {
    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        return Accepted
    }

    operator fun component1() = id
    operator fun component2() = data
}
