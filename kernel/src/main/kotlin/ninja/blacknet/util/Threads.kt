/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch

/**
 * Rotate [wheel].
 */
inline fun CoroutineScope.rotate(crossinline wheel: suspend () -> Unit): Job {
    return launch {
        while (true) {
            wheel()
        }
    }
}

/**
 * Rotate a [name]d [wheel].
 */
inline fun rotate(name: String, crossinline wheel: () -> Unit): Thread {
    return Thread.ofVirtual().name(name).start {
        while (true) {
            wheel()
        }
    }
}
