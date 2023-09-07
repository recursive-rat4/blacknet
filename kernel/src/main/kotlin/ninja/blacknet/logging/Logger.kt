/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("NOTHING_TO_INLINE")

package ninja.blacknet.logging

import io.github.oshai.kotlinlogging.KLogger

// Inline to get meaningful source class and method

inline fun KLogger.debug(throwable: Throwable) = debug(throwable) {
    throwable.debugMessage()
}

inline fun KLogger.error(throwable: Throwable) = error(throwable) {
    throwable.debugMessage()
}

inline fun KLogger.info(throwable: Throwable) = info(throwable) {
    throwable.debugMessage()
}

fun Throwable.debugMessage() = message?.let { message ->
    "${this::class.simpleName ?: this::class}: $message"
} ?: "Messageless throwable of ${this::class}"
