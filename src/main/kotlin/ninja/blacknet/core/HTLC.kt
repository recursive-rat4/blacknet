/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.contract.HashLock
import ninja.blacknet.contract.TimeLock
import ninja.blacknet.crypto.PublicKey

@Serializable
class HTLC(
        val height: Int,
        val time: Long,
        val amount: Long,
        val from: PublicKey,
        val to: PublicKey,
        val timeLock: TimeLock,
        val hashLock: HashLock
) {

}
