/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    `kotlin-dsl`
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(libs.jgit)
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_21
    }
}

val compileJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "21"
}
