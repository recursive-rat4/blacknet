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
import javax.swing.JFrame
import javax.swing.WindowConstants

fun MainWindow() = JFrame().apply {
    defaultCloseOperation = WindowConstants.EXIT_ON_CLOSE
    title = "Blacknet - Wallet"
    iconImage = Toolkit.getDefaultToolkit().getImage(Main::class.java.classLoader.getResource("logo.png"))
    jMenuBar = MainMenu()
    size = Dimension(950, 550)
    isVisible = true
}
