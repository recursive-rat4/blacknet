/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import kotlinx.coroutines.*
import mu.KotlinLogging
import ninja.blacknet.error
import ninja.blacknet.util.SynchronizedArrayList
import kotlin.coroutines.CoroutineContext

private val logger = KotlinLogging.logger {}

object Runtime : CoroutineScope {
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    private val shutdownHooks = SynchronizedArrayList<suspend () -> Unit>()

    /**
     * The number of available CPU, including virtual cores.
     */
    val availableProcessors = java.lang.Runtime.getRuntime().availableProcessors()

    /**
     * Running on macOS.
     */
    val macOS: Boolean

    /**
     * Running on Windows.
     */
    val windowsOS: Boolean

    /**
     * Registers a new shutdown hook.
     *
     * All registered shutdown hooks will be run sequentially in the reversed order.
     */
    fun addShutdownHook(hook: suspend () -> Unit) {
        runBlocking {
            shutdownHooks.add(hook)
        }
    }

    /**
     * Returns `true` if no shutdown hooks have been registered yet.
     */
    fun hasNoShutdownHooks(): Boolean {
        return runBlocking {
            shutdownHooks.isEmpty()
        }
    }

    init {
        java.lang.Runtime.getRuntime().addShutdownHook(Thread {
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
        })

        val osName = System.getProperty("os.name")
        macOS = osName.startsWith("Mac")
        windowsOS = osName.startsWith("Windows")
    }

    /**
     * Rotate [wheel].
     */
    inline fun rotate(crossinline wheel: suspend () -> Unit): Job {
        return launch {
            while (true) {
                wheel()
            }
        }
    }
}
