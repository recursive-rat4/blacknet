/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

module ninja.blacknet.serialization {
    requires java.base;

    requires kotlin.stdlib;
    requires kotlinx.serialization.core;
    requires kotlinx.serialization.json;

    requires ninja.blacknet.io;
    requires ninja.blacknet.runtime;

    exports ninja.blacknet.serialization;
    exports ninja.blacknet.serialization.bbf;
    exports ninja.blacknet.serialization.config;
}
