/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time.seconds

val Int.milliseconds get() = this / (1000 * 1L)
val Int.seconds get() = toLong()
val Int.minutes get() = this * (60 * 1L)
val Int.hours get() = this * (60 * 60 * 1L)
val Int.days get() = this * (24 * 60 * 60 * 1L)
