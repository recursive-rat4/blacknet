/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time.milliseconds

import kotlin.random.Random

fun Random.nextTime(): MilliSeconds {
    return MilliSeconds(nextLong())
}

fun Random.nextTime(until: MilliSeconds): MilliSeconds {
    return MilliSeconds(nextLong(until.milliseconds))
}

fun Random.nextTime(from: MilliSeconds, until: MilliSeconds): MilliSeconds {
    return MilliSeconds(nextLong(from.milliseconds, until.milliseconds))
}
