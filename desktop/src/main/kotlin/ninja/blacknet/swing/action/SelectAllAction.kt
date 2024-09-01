/*
 * Copyright (c) 2023-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("CONTEXT_RECEIVERS_DEPRECATED")

package ninja.blacknet.swing.action

import java.awt.event.ActionEvent
import java.awt.event.KeyEvent.*
import javax.swing.AbstractAction
import javax.swing.text.JTextComponent
import ninja.blacknet.swing.dsl.*

context(JTextComponent)
class SelectAllAction : AbstractAction() {
    init {
        name = "Select all"
        accelerator = KeyStroke(VK_A, CTRL_DOWN_MASK)
    }

    override fun actionPerformed(e: ActionEvent) {
        selectAll()
        // selection ain't automatically visible if component never was focused
        caret.isSelectionVisible = true
    }
}
