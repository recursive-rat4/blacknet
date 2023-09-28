/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.swing.action

import java.awt.event.ActionEvent
import java.awt.event.KeyEvent.*
import javax.swing.AbstractAction
import ninja.blacknet.swing.dsl.*

object QuitAction : AbstractAction() {
    init {
        name = "Quit"
        accelerator = KeyStroke(VK_Q, CTRL_DOWN_MASK)
    }

    override fun actionPerformed(e: ActionEvent) {
        System.exit(0)
    }
}
