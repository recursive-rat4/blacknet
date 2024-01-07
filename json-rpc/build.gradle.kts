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
    alias(libs.plugins.kotlin.serialization)
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":blacknet-serialization"))
    api(libs.kotlin.stdlib)
    api(libs.kotlin.serialization)
    api(libs.kotlin.serialization.json)
    testImplementation(libs.kotlin.testng) {
        exclude("aopalliance", "aopalliance")
        exclude("junit", "junit")
    }
}

val compileJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "19"
}

val compileTestJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "19"
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_19
        freeCompilerArgs = listOf(
            "-Xexplicit-api=strict",
            "-Xjvm-default=all",
            *optOut(project),
        )
    }
}

val compileTestKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_19
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
