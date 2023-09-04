/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.event.ActionEvent
import javax.swing.AbstractAction
import ninja.blacknet.swing.dsl.*

class HideOnCloseAction : AbstractAction() {
    init {
        name = "Hide on close"
        toolTipText = "When the main window is closed, the tray icon stays."
        selected = Config.hideOnClose
    }

    override fun actionPerformed(e: ActionEvent) {
        Config.hideOnClose = !Config.hideOnClose
    }
}
