/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

module ninja.blacknet.runtime {
    requires java.base;

    requires kotlin.stdlib;

    exports ninja.blacknet;
    exports ninja.blacknet.codec.base;
    exports ninja.blacknet.util;
}
