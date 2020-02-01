/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.selects.SelectBuilder
import kotlinx.coroutines.withTimeout
import kotlinx.coroutines.withTimeoutOrNull
import ninja.blacknet.time.milliseconds.MilliSeconds

suspend inline fun delay(time: MilliSeconds) {
    return delay(time.milliseconds)
}

suspend inline fun <T> withTimeout(time: MilliSeconds, noinline block: suspend CoroutineScope.() -> T): T {
    return withTimeout(time.milliseconds, block)
}

suspend inline fun <T> withTimeoutOrNull(time: MilliSeconds, noinline block: suspend CoroutineScope.() -> T): T? {
    return withTimeoutOrNull(time.milliseconds, block)
}

@Suppress("NOTHING_TO_INLINE")
inline fun <R> SelectBuilder<R>.onTimeout(time: MilliSeconds, noinline block: suspend () -> R) {
    return onTimeout(time.milliseconds, block)
}
