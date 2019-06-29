/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import ninja.blacknet.util.SynchronizedArrayList
import kotlin.coroutines.CoroutineContext

object Runtime : CoroutineScope {
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    private val shutdownHooks = SynchronizedArrayList<suspend () -> Unit>()

    /**
     * The number of available CPU, including virtual cores.
     */
    val availableProcessors = java.lang.Runtime.getRuntime().availableProcessors()

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

    init {
        java.lang.Runtime.getRuntime().addShutdownHook(Thread {
            runBlocking {
                shutdownHooks.reversedForEach {
                    it()
                }
            }
        })
    }
}
