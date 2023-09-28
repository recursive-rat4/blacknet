/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.window.main

import javax.swing.table.AbstractTableModel

class AddressBookTableModel : AbstractTableModel() {
    private val columns = arrayOf("Label", "Address")

    override fun getColumnCount(): Int {
        return columns.size
    }

    override fun getColumnName(column: Int): String {
        return columns[column]
    }

    override fun getRowCount(): Int {
        return 0
    }

    override fun getValueAt(rowIndex: Int, columnIndex: Int): Any {
        throw IndexOutOfBoundsException("($rowIndex,$columnIndex) not in table")
    }

    override fun isCellEditable(rowIndex: Int, columnIndex: Int): Boolean {
        return true
    }

    override fun setValueAt(aValue: Any?, rowIndex: Int, columnIndex: Int) {
        throw IndexOutOfBoundsException("($rowIndex,$columnIndex) not in table")
    }
}
