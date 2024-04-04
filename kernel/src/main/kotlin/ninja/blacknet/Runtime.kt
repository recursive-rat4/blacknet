/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

object Runtime {
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

    init {
        System.getProperty("os.name").let { os_name ->
            macOS = os_name.startsWith("Mac")
            windowsOS = os_name.startsWith("Windows")
        }
    }
}
