/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.EventQueue
import java.awt.Window
import javax.swing.SwingUtilities
import javax.swing.UIManager
import ninja.blacknet.swing.dsl.*

fun LookAndFeelComboBox() = jComboBox {
    UIManager.getInstalledLookAndFeels().forEach { info ->
        +info.name
    }
    selectedItem = UIManager.getLookAndFeel().getName()
    addActionListener {
        val name = selectedItem as String
        UIManager.setLookAndFeel(UIManager.createLookAndFeel(name))
        // invoke later to work around an exception when triggered from keyboard
        EventQueue.invokeLater {
            Window.getWindows().forEach { window ->
                SwingUtilities.updateComponentTreeUI(window)
            }
        }
    }
}
