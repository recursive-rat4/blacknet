/*
 * Copyright (c) 2019-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.MenuItem
import java.awt.PopupMenu
import java.awt.SystemTray
import java.awt.Toolkit
import java.awt.TrayIcon

fun TrayIcon() {
    val image = Toolkit.getDefaultToolkit().getImage(Desktop::class.java.classLoader.getResource("logo.png"))
    val quitItem = MenuItem("Quit")
    quitItem.addActionListener(QuitAction)
    val popup = PopupMenu()
    popup.add(quitItem)
    val trayIcon = TrayIcon(image, "Blacknet", popup)
    trayIcon.isImageAutoSize = true
    trayIcon.addActionListener {
        MainWindow.isVisible = !MainWindow.isVisible
    }
    SystemTray.getSystemTray().add(trayIcon)
}
