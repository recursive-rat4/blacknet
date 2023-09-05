/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import kotlinx.serialization.Serializable

@Serializable
class WalletTransactionInfo(
        val transaction: TransactionInfo,
        val confirmations: Int,
        val receiveTime: Long
)
