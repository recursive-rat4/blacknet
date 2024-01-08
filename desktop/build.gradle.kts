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
    alias(libs.plugins.kotlin.jvm)
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":blacknet-runtime"))
    implementation(libs.kotlin.stdlib)
    implementation(libs.guava)
}

application {
    mainClass = "ninja.blacknet.Desktop"
}

val compileJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "20"
}

val compileTestJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "20"
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_20
        freeCompilerArgs = listOf(
            "-Xcontext-receivers",
            "-Xjvm-default=all",
            *optOut(project),
        )
    }
}

val compileTestKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_20
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
