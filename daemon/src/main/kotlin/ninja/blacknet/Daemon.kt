/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import io.github.oshai.kotlinlogging.KotlinLogging
import ninja.blacknet.logging.error
import ninja.blacknet.network.ChainFetcher

private val logger = KotlinLogging.logger {}

object Daemon {
    @JvmStatic
    fun main(args: Array<String>) {
        Kernel.init(args) { _, e ->
            logger.error(e)
        }

        try {
            ChainFetcher.join()
        } catch (e: Throwable) {
            logger.error(e)
        } finally {
            System.exit(1) //FIXME non-main threads shouldn't inhibit shutdown
        }
    }
}
