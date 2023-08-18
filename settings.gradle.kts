/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

rootProject.name = "blacknet"

include(":blacknet-benchmarks")
project(":blacknet-benchmarks").projectDir = file("benchmarks")

include(":blacknet-cli")
project(":blacknet-cli").projectDir = file("cli")

include(":blacknet-daemon")
project(":blacknet-daemon").projectDir = file("daemon")

include(":blacknet-desktop")
project(":blacknet-desktop").projectDir = file("desktop")

include(":blacknet-json-rpc")
project(":blacknet-json-rpc").projectDir = file("json-rpc")

include(":blacknet-runtime")
project(":blacknet-runtime").projectDir = file("runtime")

include(":blacknet-serialization")
project(":blacknet-serialization").projectDir = file("serialization")

include(":blacknet-time")
project(":blacknet-time").projectDir = file("time")

pluginManagement {
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}
