/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm")
}

repositories {
    mavenCentral()
}

dependencies {
    api("org.jetbrains.kotlin:kotlin-stdlib:${Versions.kotlin}")
    api("org.jetbrains.kotlinx:kotlinx-serialization-core:${Versions.serialization}")
    api("org.jetbrains.kotlinx:kotlinx-serialization-json:${Versions.serialization}")
    testImplementation("org.jetbrains.kotlin:kotlin-test-testng:${Versions.kotlin}") {
        exclude("aopalliance", "aopalliance")
        exclude("junit", "junit")
    }
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    kotlinOptions {
        jvmTarget = "11"
        freeCompilerArgs = listOf(
                "-Xexplicit-api=strict",
                "-Xjvm-default=all"
        )
    }
}

val compileTestKotlin by tasks.existing(KotlinCompile::class) {
    kotlinOptions {
        jvmTarget = "11"
    }
}

val jar by tasks.existing(Jar::class) {
    manifest {
        attributes(
                "Implementation-Title" to project.name.toString(),
                "Implementation-Vendor" to "Blacknet Team",
                "Implementation-Version" to project.version.toString()
        )
    }
}

val test by tasks.existing(Test::class) {
    useTestNG()
}
