/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.FlowLayout.LEADING
import ninja.blacknet.swing.dsl.*

fun AddressBookPanel() = jPanel {
    name = "Address book"
    borderLayout {
        center = jScrollPane {
            viewport.view = jTable {
                autoCreateRowSorter = true
                model = AddressBookTableModel()
            }
        }
        south = jPanel {
            flowLayout {
                alignment = LEADING
                +jButton {
                    text = "New"
                }
                +jButton {
                    text = "Copy"
                }
                +jButton {
                    text = "Delete"
                }
            }
        }
    }
}
