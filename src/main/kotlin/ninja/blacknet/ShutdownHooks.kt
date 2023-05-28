/*
 * Copyright (c) 2019-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.lang.Runtime
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.withLock
import mu.KotlinLogging
import ninja.blacknet.logging.error
import ninja.blacknet.util.SynchronizedArrayList

private val logger = KotlinLogging.logger {}

object ShutdownHooks {
    @Suppress("JoinDeclarationAndAssignment")
    private val shutdownHooks: SynchronizedArrayList<() -> Unit>

    init {
        shutdownHooks = SynchronizedArrayList()
        Runtime.getRuntime().addShutdownHook(Executor())
    }

    /**
     * Registers a new shutdown hook.
     *
     * All registered shutdown hooks will be run sequentially in the reversed order.
     */
    fun add(hook: () -> Unit) {
        runBlocking {
            shutdownHooks.mutex.withLock {
                shutdownHooks.list.add(hook)
            }
        }
    }

    private class Executor() : Thread() {
        override fun run() {
            logger.info("Shutdown is in progress...")
            runBlocking {
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
