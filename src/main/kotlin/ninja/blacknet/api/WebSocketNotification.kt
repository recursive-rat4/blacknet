/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement

@Serializable
class WebSocketNotification(
        val route: String,
        val message: JsonElement
) {
    constructor(notification: BlockNotification) : this(
            "block",
            notification.toJson()
    )

    constructor(notification: TransactionNotification) : this(
            "transaction",
            notification.toJson()
    )
}
