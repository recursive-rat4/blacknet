/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.util.concurrent.locks.Lock

inline fun <T> Lock.withUnlock(action: () -> T): T {
    // Experimental contracts
    // import kotlin.contracts.*
    // contract {
    //     callsInPlace(action, InvocationKind.EXACTLY_ONCE)
    // }
    unlock()
    try {
        return action()
    } finally {
        lock()
    }
}
