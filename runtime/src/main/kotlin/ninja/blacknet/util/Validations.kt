/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

/**
 * Checks whether a [condition] is satisfied and if it is not then throws an exception returned by [lazyThrowable].
 */
public inline fun check(condition: Boolean, lazyThrowable: () -> Throwable) {
    // contract {
    //     returns() implies condition
    // }
    if (condition)
        return
    else
        throw lazyThrowable()
}
