/*
 * Copyright (c) 2018-2024 Pavel Vasin
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
    alias(libs.plugins.kotlin.allopen)
    kotlin("kapt")
    application
}

repositories {
    mavenCentral()
}

allOpen {
    annotation("org.openjdk.jmh.annotations.State")
}

dependencies {
    implementation(project(":blacknet-kernel"))
    implementation(project(":blacknet-serialization"))
    implementation(libs.kotlin.serialization)
    implementation(libs.jmh.core)
    kapt(libs.jmh.generator.annprocess)
}

application {
    mainClass = "org.openjdk.jmh.Main"
}

val compileJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "21"
}

val compileTestJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "21"
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_21
        freeCompilerArgs = listOf(
            "-Xjvm-default=all",
            *optOut(project),
        )
    }
}

val compileTestKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_21
    }
}
