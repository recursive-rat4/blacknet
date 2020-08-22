/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

plugins {
    kotlin("jvm")
}

repositories {
    jcenter()
}

dependencies {
    api("org.jetbrains.kotlin:kotlin-stdlib:${Versions.kotlin}")
}

val jar by tasks.existing(Jar::class) {
    manifest {
        attributes(
            "Implementation-Version" to project.version.toString()
        )
    }
}
