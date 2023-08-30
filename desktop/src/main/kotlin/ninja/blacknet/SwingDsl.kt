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
import javax.swing.*
import javax.swing.Action.*

inline fun jCheckBox(build: JCheckBox.() -> Unit) = JCheckBox().apply(build)
inline fun <T> jComboBox(build: JComboBox<T>.() -> Unit) = JComboBox<T>().apply(build)
inline fun jDialog(build: JDialog.() -> Unit) = JDialog().apply(build)
inline fun jFrame(build: JFrame.() -> Unit) = JFrame().apply(build)
inline fun jMenu(build: JMenu.() -> Unit) = JMenu().apply(build)
inline fun jMenuBar(build: JMenuBar.() -> Unit) = JMenuBar().apply(build)
inline fun jPanel(build: JPanel.() -> Unit) = JPanel(false).apply(build)
inline fun jScrollPane(build: JScrollPane.() -> Unit) = JScrollPane().apply(build)
inline fun jTabbedPane(build: JTabbedPane.() -> Unit) = JTabbedPane().apply(build)
inline fun jTextArea(build: JTextArea.() -> Unit) = JTextArea().apply(build)

context(Container)
operator fun Component.unaryPlus() {
    this@Container.add(this@Component)
}

context(JComboBox<T>)
operator fun <T> T.unaryPlus() {
    this@JComboBox.addItem(this@T)
}

context(JMenu)
operator fun Action.unaryPlus() {
    this@JMenu.add(this@Action)
}

var Action.name: String?
    get() = getValue(NAME) as String?
    set(value) = putValue(NAME, value)

var Action.selected: Boolean?
    get() = getValue(SELECTED_KEY) as Boolean?
    set(value) = putValue(SELECTED_KEY, value)

var Action.toolTipText: String?
    get() = getValue(SHORT_DESCRIPTION) as String?
    set(value) = putValue(SHORT_DESCRIPTION, value)
