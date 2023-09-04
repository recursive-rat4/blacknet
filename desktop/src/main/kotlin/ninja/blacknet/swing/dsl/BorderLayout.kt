/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.swing.dsl

import java.awt.BorderLayout
import java.awt.Component
import java.awt.Container

inline fun Container.borderLayout(build: BorderLayout.() -> Unit) {
    BorderLayout().apply {
        layout = this
        build()
    }
}

context(BorderLayout)
var Container.center: Component?
    get() = getLayoutComponent(BorderLayout.CENTER)
    set(value) {
        if (value != null)
            add(value, BorderLayout.CENTER)
        else
            remove(center)
    }

context(BorderLayout)
var Container.south: Component?
    get() = getLayoutComponent(BorderLayout.SOUTH)
    set(value) {
        if (value != null)
            add(value, BorderLayout.SOUTH)
        else
            remove(south)
    }
