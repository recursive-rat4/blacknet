/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

rootProject.name = "blacknet"

pluginManagement {
    repositories {
        jcenter()
        gradlePluginPortal()
        maven {
            name = "Kotlin Early Access Preview"
            url = uri("https://dl.bintray.com/kotlin/kotlin-eap")
        }
    }
}
