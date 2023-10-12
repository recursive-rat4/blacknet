/*
 * Copyright (c) 2020-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import kotlin.system.exitProcess
import ninja.blacknet.Mode.*

/**
 * An enumeration of implemented modes: production or various research, development, testing.
 *
 * @property subdirectory to separate data.
 * @property addressPrefix to designate a different ledger.
 * @property agentSuffix for network indication.
 * @property requiresNetwork whether the node is supposed to have peers.
 */
enum class Mode(
    val subdirectory: String?,
    val addressPrefix: String?,
    val agentSuffix: String?,
    val requiresNetwork: Boolean,
) {
    /**
     * The main network. It's the production mode.
     */
    MainNet(
        null,
        null,
        null,
        true,
    ),

    TestNet(
        "TestNet",
        "t",
        "TestNet",
        true
    ),

    SigNet(
        "SigNet",
        "s",
        "SigNet",
        true
    ),

    /**
     * A regression testing mode. Usually it's a sole offline node, or else it can be a tiny private network.
     */
    RegTest(
        "RegTest",
        "r",
        "RegTest",
        false,
    ),
}

/**
 * A [Mode] the program is running in. By default, it's [MainNet].
 */
val mode: Mode = when (val property = System.getProperty("ninja.blacknet.mode")) {
    null -> MainNet
    "MainNet" -> MainNet
    "TestNet" -> notImplementedError("TestNet was not tested")
    "SigNet" -> notImplementedError("SigNet was not signed")
    "RegTest" -> RegTest
    else -> notImplementedError("Unrecognized mode: $property. Possible values: MainNet, RegTest.")
}

private fun notImplementedError(message: String): Nothing {
    // logging is not yet available
    System.err.println(message)
    exitProcess(1)
}
