/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import javax.swing.Action
import javax.swing.JMenu
import javax.swing.JMenuBar

fun MainMenu() = JMenuBar().apply {
    +JMenu().apply {
        text = "File"
        +QuitAction
    }
    +JMenu().apply {
        text = "Edit"
    }
    +JMenu().apply {
        text = "Help"
    }
}

context(JMenu)
private operator fun Action.unaryPlus() {
    this@JMenu.add(this@Action)
}

context(JMenuBar)
private operator fun JMenu.unaryPlus() {
    this@JMenuBar.add(this@JMenu)
}
