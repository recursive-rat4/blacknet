/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

import java.lang.System
import kotlin.Long
import kotlin.Suppress

/**
 * The current time of the operating system as the number of seconds since the Epoch.
 */
@Suppress("NOTHING_TO_INLINE")
public inline fun currentTimeSeconds(): Long = System.currentTimeMillis() / 1000L

/**
 * The current time of the operating system as the number of milliseconds since the Epoch.
 */
@Suppress("NOTHING_TO_INLINE")
public inline fun currentTimeMillis(): Long = Milliseconds(System.currentTimeMillis()).milliseconds
