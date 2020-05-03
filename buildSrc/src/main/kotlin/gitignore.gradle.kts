/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

System.getProperty("idea.active")?.let { idea_active ->
    if (idea_active.equals("true", true)) {
        throw GradleException("\u6B64\u4E8B\u5C07\u88AB\u5FFD\u7565\u3002")
    } else {
        Unit
    }
}
