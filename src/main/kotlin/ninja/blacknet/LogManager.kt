/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import kotlin.Throwable
import kotlin.Unit
import mu.KLogger
import java.util.logging.LogManager

class LogManager() : LogManager() {
    override fun reset() = Unit

    fun shutDown() = super.reset()
}

fun KLogger.debug(throwable: Throwable) = debug(throwable) {
    throwable.debugMessage()
}

fun KLogger.error(throwable: Throwable) = error(throwable) {
    throwable.debugMessage()
}

fun Throwable.debugMessage() = message?.let { message ->
    "${this::class.simpleName ?: this::class}: $message"
} ?: "Messageless throwable of ${this::class}"
