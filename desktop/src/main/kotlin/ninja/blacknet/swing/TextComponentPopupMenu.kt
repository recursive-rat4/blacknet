/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.swing

import javax.swing.text.JTextComponent
import ninja.blacknet.swing.action.CopyAction
import ninja.blacknet.swing.action.CutAction
import ninja.blacknet.swing.action.DeleteAction
import ninja.blacknet.swing.action.PasteAction
import ninja.blacknet.swing.action.SelectAllAction
import ninja.blacknet.swing.dsl.*

context(JTextComponent)
fun TextComponentPopupMenu() = jPopupMenu {
    +CutAction()
    +CopyAction()
    +PasteAction()
    +DeleteAction()
    addSeparator()
    +SelectAllAction()
}
