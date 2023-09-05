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

apply<kotlinx.atomicfu.plugin.gradle.AtomicFUGradlePlugin>()

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":blacknet-json-rpc"))
    implementation(project(":blacknet-runtime"))
    implementation(project(":blacknet-serialization"))
    implementation(project(":blacknet-time"))
    implementation(libs.kotlin.stdlib)
    implementation(libs.ktor.cio)
    implementation(libs.ktor.default.headers)
    implementation(libs.ktor.status.pages)
    implementation(libs.ktor.io)
    implementation(libs.ktor.network)
    implementation(libs.ktor.websockets)
    implementation(libs.kotlin.coroutines)
    implementation(libs.kotlin.coroutines.debug)
    implementation(libs.kotlin.serialization)
    implementation(libs.kotlin.serialization.json)
    implementation(libs.apache.collections)
    implementation(libs.eddsa)
    implementation(libs.blake2b)
    implementation(libs.bouncycastle)
    implementation(libs.leveldb.java)
    implementation(libs.slf4j)
    implementation(libs.kotlin.logging)
    implementation(libs.guava)
    implementation(libs.weupnp)
    implementation(files("../buildSrc/libs/leveldbjni-all-1.18.3.jar"))
    testImplementation(libs.kotlin.testng) {
        exclude("aopalliance", "aopalliance")
        exclude("junit", "junit")
    }
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
        dirtyDescribeGit(buildDir.getParentFile().getParentFile())?.let { revision ->
            attributes("Build-Revision" to revision)
        }
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
