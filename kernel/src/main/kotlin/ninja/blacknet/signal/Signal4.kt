/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("INLINE_CLASS_DEPRECATED")

package ninja.blacknet.signal

inline class Signal4<A1, A2, A3, A4>(
    private val slots: ArrayList<(A1, A2, A3, A4) -> Unit> = ArrayList()
) {
    fun connect(slot: (A1, A2, A3, A4) -> Unit) = synchronized(slots) {
        slots.add(slot)
    }

    fun disconnect(slot: (A1, A2, A3, A4) -> Unit) = synchronized(slots) {
        slots.remove(slot)
    }

    operator fun invoke(a1: A1, a2: A2, a3: A3, a4: A4) = synchronized(slots) {
        slots.forEach { it(a1, a2, a3, a4) }
    }
}
