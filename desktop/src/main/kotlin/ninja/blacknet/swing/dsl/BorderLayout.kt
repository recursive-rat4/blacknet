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
import java.awt.BorderLayout.*
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
    get() = getLayoutComponent(CENTER)
    set(value) {
        if (value != null)
            add(value, CENTER)
        else
            remove(center)
    }

context(BorderLayout)
var Container.north: Component?
    get() = getLayoutComponent(NORTH)
    set(value) {
        if (value != null)
            add(value, NORTH)
        else
            remove(north)
    }

context(BorderLayout)
var Container.south: Component?
    get() = getLayoutComponent(SOUTH)
    set(value) {
        if (value != null)
            add(value, SOUTH)
        else
            remove(south)
    }

context(BorderLayout)
var Container.west: Component?
    get() = getLayoutComponent(WEST)
    set(value) {
        if (value != null)
            add(value, WEST)
        else
            remove(west)
    }

context(BorderLayout)
var Container.east: Component?
    get() = getLayoutComponent(EAST)
    set(value) {
        if (value != null)
            add(value, EAST)
        else
            remove(east)
    }
