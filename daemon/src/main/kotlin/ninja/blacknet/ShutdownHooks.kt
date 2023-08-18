/*
 * Copyright (c) 2019-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import io.github.oshai.kotlinlogging.KotlinLogging
import java.lang.Runtime
import ninja.blacknet.logging.error

private val logger = KotlinLogging.logger {}

object ShutdownHooks {
    @Suppress("JoinDeclarationAndAssignment")
    private val shutdownHooks: ArrayList<() -> Unit>

    init {
        shutdownHooks = ArrayList()
        Runtime.getRuntime().addShutdownHook(Executor())
    }

    /**
     * Registers a new shutdown hook.
     *
     * All registered shutdown hooks will be run sequentially in the reversed order.
     */
    fun add(hook: () -> Unit) {
        synchronized(shutdownHooks) {
            shutdownHooks.add(hook)
        }
    }

    private class Executor() : Thread() {
        override fun run() {
            logger.info { "Shutdown is in progress..." }
            synchronized(shutdownHooks) {
                shutdownHooks.reversedForEach { hook ->
                    try {
                        hook()
                    } catch (e: Throwable) {
                        logger.error(e)
                    }
                }
            }
        }
    }
}

private inline fun <T> ArrayList<T>.reversedForEach(action: (T) -> Unit) {
    for (i in size - 1 downTo 0)
        action(get(i))
}
