/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.window.main

import java.awt.event.KeyEvent.*
import ninja.blacknet.swing.action.QuitAction
import ninja.blacknet.swing.dsl.*

fun MainMenu() = jMenuBar {
    +jMenu {
        text = "File"
        mnemonic = VK_F
        +QuitAction
    }
    +jMenu {
        text = "Edit"
        mnemonic = VK_E
        +PreferencesAction()
    }
    +jMenu {
        text = "Help"
        mnemonic = VK_H
    }
}
