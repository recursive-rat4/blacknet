/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.Component
import java.awt.Container
import javax.swing.Action
import javax.swing.JDialog
import javax.swing.JFrame
import javax.swing.JMenu
import javax.swing.JMenuBar
import javax.swing.JPanel
import javax.swing.JScrollPane
import javax.swing.JTabbedPane

inline fun jDialog(build: JDialog.() -> Unit) = JDialog().apply(build)
inline fun jFrame(build: JFrame.() -> Unit) = JFrame().apply(build)
inline fun jMenu(build: JMenu.() -> Unit) = JMenu().apply(build)
inline fun jMenuBar(build: JMenuBar.() -> Unit) = JMenuBar().apply(build)
inline fun jPanel(build: JPanel.() -> Unit) = JPanel(false).apply(build)
inline fun jScrollPane(build: JScrollPane.() -> Unit) = JScrollPane().apply(build)
inline fun jTabbedPane(build: JTabbedPane.() -> Unit) = JTabbedPane().apply(build)

context(Container)
operator fun Component.unaryPlus() {
    this@Container.add(this@Component)
}

context(JMenu)
operator fun Action.unaryPlus() {
    this@JMenu.add(this@Action)
}

var Action.name: String?
    get() = getValue(Action.NAME) as String?
    set(value) = putValue(Action.NAME, value)
