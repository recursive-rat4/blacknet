/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api.v1

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.PaymentId
import ninja.blacknet.transaction.Transfer

@Serializable
class TransferInfo(
        val amount: Long,
        val to: String,
        val message: String
) {
    constructor(data: Transfer) : this(
            data.amount,
            Address.encode(data.to),
            when (data.message.type) {
                PaymentId.PLAIN -> String(data.message.message)
                PaymentId.ENCRYPTED -> "ENCRYPTED:${data.message.message}"
                else -> "UNKNOWN TYPE:${data.message.type} DATA:${data.message.message}"
            }
    )
}
