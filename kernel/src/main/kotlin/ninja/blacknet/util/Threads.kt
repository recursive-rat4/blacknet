/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

/**
 * Rotate a [name]d [wheel].
 */
inline fun rotate(name: String, crossinline wheel: () -> Unit): Thread {
    return startInterruptible(name) {
        while (true) {
            wheel()
        }
    }
}

/**
 * Run an interruption-safe [block] in a new [name]d virtual thread.
 */
inline fun startInterruptible(name: String, crossinline block: () -> Unit): Thread {
    return Thread.ofVirtual().name(name).start {
        interruptible {
            block()
        }
    }
}

/**
 * Run an interruption-safe [block].
 */
inline fun interruptible(block: () -> Unit) {
    try {
        block()
    } catch (e: InterruptedException) {
        // Interruptible
    }
}
