/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.window.main

import ninja.blacknet.swing.dsl.*

fun AtomicSwapPane() = jTabbedPane {
    name = "Atomic swap"
    +jPanel {
        name = "Create"
        borderLayout {
            south = jPanel {
                borderLayout {
                    east = jButton {
                        text = "Send"
                    }
                }
            }
        }
    }
    +jPanel {
        name = "Claim"
        borderLayout {
            center = jScrollPane {
                viewport.view = jTable {
                }
            }
            south = jPanel {
                borderLayout {
                    west = jLabel {
                        text = "Preimage"
                    }
                    center = jTextField {
                    }
                    east = jButton {
                        text = "Send"
                    }
                }
            }
        }
    }
    +jPanel {
        name = "Refund"
        borderLayout {
            center = jScrollPane {
                viewport.view = jTable {
                }
            }
            south = jPanel {
                borderLayout {
                    east = jButton {
                        text = "Send"
                    }
                }
            }
        }
    }
}
