/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.Dimension
import java.awt.Toolkit
import java.awt.event.WindowAdapter
import java.awt.event.WindowEvent
import javax.swing.JFrame
import javax.swing.JTabbedPane.LEFT

object MainWindow : JFrame() {
    init {
        defaultCloseOperation = DO_NOTHING_ON_CLOSE
        title = "Blacknet - Wallet"
        iconImage = Toolkit.getDefaultToolkit().getImage(Main::class.java.classLoader.getResource("logo.png"))
        jMenuBar = MainMenu()
        size = Dimension(950, 550)
        contentPane = jTabbedPane {
            tabPlacement = LEFT
            +jPanel {
                name = "Dashboard"
            }
            +jPanel {
                name = "Transfer"
            }
            +jPanel {
                name = "Atomic swap"
            }
            +HistoryPane()
            +jPanel {
                name = "Leasing"
            }
            +jPanel {
                name = "Staking"
            }
            +jPanel {
                name = "Address book"
            }
        }
        addWindowListener(object : WindowAdapter() {
            override fun windowClosing(e: WindowEvent) {
                if (Config.hideOnClose)
                    isVisible = false
                else
                    System.exit(0)
            }
        })
        isVisible = true
    }
}
