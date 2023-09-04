/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.google.common.base.Throwables
import ninja.blacknet.swing.dsl.*

fun ExceptionDialog(
    e: Throwable
) = ErrorDialog(
    arrayOf(
        e.localizedMessage,
        jScrollPane {
            viewport.view = jTextArea {
                columns = TERMINAL_WIDTH
                rows = TERMINAL_HEIGHT
                text = Throwables.getStackTraceAsString(e)
                caretPosition = 0
                isEditable = false
            }
        },
    )
)
