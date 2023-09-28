/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.awt.EventQueue
import ninja.blacknet.window.ExceptionDialog
import ninja.blacknet.window.main.MainWindow

object Desktop {
    @JvmStatic
    fun main(args: Array<String>) {
        Thread.setDefaultUncaughtExceptionHandler { _, e ->
            EventQueue.invokeLater {
                ExceptionDialog(e)
            }
        }

        EventQueue.invokeLater {
            TrayIcon()
            MainWindow
        }
    }
}
