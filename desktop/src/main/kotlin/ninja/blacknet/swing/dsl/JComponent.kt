/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.swing.dsl

import java.awt.Component
import java.awt.Container
import javax.swing.*

inline fun jButton(build: JButton.() -> Unit) = JButton().apply(build)
inline fun jCheckBox(build: JCheckBox.() -> Unit) = JCheckBox().apply(build)
inline fun <T> jComboBox(build: JComboBox<T>.() -> Unit) = JComboBox<T>().apply(build)
inline fun jDialog(build: JDialog.() -> Unit) = JDialog().apply(build)
inline fun jFrame(build: JFrame.() -> Unit) = JFrame().apply(build)
inline fun jLabel(build: JLabel.() -> Unit) = JLabel().apply(build)
inline fun jMenu(build: JMenu.() -> Unit) = JMenu().apply(build)
inline fun jMenuBar(build: JMenuBar.() -> Unit) = JMenuBar().apply(build)
inline fun jPanel(build: JPanel.() -> Unit) = JPanel(false).apply(build)
inline fun jScrollPane(build: JScrollPane.() -> Unit) = JScrollPane().apply(build)
inline fun jTabbedPane(build: JTabbedPane.() -> Unit) = JTabbedPane().apply(build)
inline fun jTable(build: JTable.() -> Unit) = JTable().apply(build)
inline fun jTextArea(build: JTextArea.() -> Unit) = JTextArea().apply(build)

context(Container)
operator fun Component.unaryPlus() {
    this@Container.add(this@Component)
}

context(JComboBox<T>)
operator fun <T> T.unaryPlus() {
    this@JComboBox.addItem(this@T)
}
