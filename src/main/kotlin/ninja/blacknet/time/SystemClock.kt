/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

import ninja.blacknet.time.milliseconds.MilliSeconds

/**
 * Implementation of a clock that returns the current time of the operating system.
 */
object SystemClock {
    /**
     * The current time as the number of seconds since the Epoch.
     */
    val seconds: Long
        get() = System.currentTimeMillis() / 1000

    /**
     * The current time as the number of milliseconds since the Epoch.
     */
    val milliseconds: MilliSeconds
        get() = MilliSeconds(System.currentTimeMillis())
}
