/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.window.preferences

import java.awt.Dimension
import javax.swing.WindowConstants.DISPOSE_ON_CLOSE
import ninja.blacknet.swing.dsl.*

fun PreferencesDialog() = jDialog {
    defaultCloseOperation = DISPOSE_ON_CLOSE
    isModal = true
    title = "Blacknet - Preferences"
    size = Dimension(540, 380)
    contentPane = jTabbedPane {
        +jPanel {
            name = "Interface"
            +LookAndFeelComboBox()
            +jCheckBox {
                action = HideOnCloseAction()
            }
            +jCheckBox {
                action = HideOnMinimizeAction()
            }
        }
    }
    isVisible = true
}
