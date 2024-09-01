/*
 * Copyright (c) 2023-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("CONTEXT_RECEIVERS_DEPRECATED")

package ninja.blacknet.swing.dsl

import javax.swing.Action
import javax.swing.Action.*
import javax.swing.JMenu
import javax.swing.JPopupMenu
import javax.swing.KeyStroke

context(JMenu)
operator fun Action.unaryPlus() {
    this@JMenu.add(this@Action)
}

context(JPopupMenu)
operator fun Action.unaryPlus() {
    this@JPopupMenu.add(this@Action)
}

var Action.accelerator: KeyStroke?
    get() = getValue(ACCELERATOR_KEY) as KeyStroke?
    set(value) = putValue(ACCELERATOR_KEY, value)

var Action.name: String?
    get() = getValue(NAME) as String?
    set(value) = putValue(NAME, value)

var Action.selected: Boolean?
    get() = getValue(SELECTED_KEY) as Boolean?
    set(value) = putValue(SELECTED_KEY, value)

var Action.toolTipText: String?
    get() = getValue(SHORT_DESCRIPTION) as String?
    set(value) = putValue(SHORT_DESCRIPTION, value)
