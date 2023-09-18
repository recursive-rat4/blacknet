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
    implementation(project(":blacknet-kernel"))
    implementation(libs.kotlin.stdlib)
    implementation(libs.kotlin.logging)
}

application {
    mainClass = "ninja.blacknet.Daemon"
}

val compileJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "17"
}

val compileTestJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "17"
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_17
        freeCompilerArgs = listOf(
            "-Xjvm-default=all",
            *optOut(project),
        )
    }
}

val compileTestKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_17
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

val run by tasks.existing(JavaExec::class) {
    classpath = files(tasks.jar) + classpath.filter { !it.startsWith(buildDir) }
    systemProperties = defaultSystemProperties
}

val startScripts by tasks.existing(CreateStartScripts::class) {
    defaultJvmOpts = defaultSystemProperties.map { (key, value) -> "-D$key=$value" }
}

val defaultSystemProperties: Map<String, Any> = mapOf(
    // Output hexadecimal values with lower case letters
    // "ninja.blacknet.codec.base.hex.lowercase" to true,
    // Indent JSON returned by RPC API
    // "ninja.blacknet.serialization.json.indented" to true,
    // Regression testing mode
    // "ninja.blacknet.mode" to "RegTest",
)
