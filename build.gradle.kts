/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

import nl.javadude.gradle.plugins.license.DownloadLicenses
import nl.javadude.gradle.plugins.license.LicenseMetadata
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

allprojects {
    group = "ninja.blacknet"
    version = "0.3-SNAPSHOT"
}

buildscript {
    dependencies {
        classpath(libs.kotlin.atomicfu)
    }
}

plugins {
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.kotlin.serialization)
    application
    distribution
    alias(libs.plugins.licenses)
}

apply<kotlinx.atomicfu.plugin.gradle.AtomicFUGradlePlugin>()

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":blacknet-runtime"))
    implementation(project(":blacknet-serialization"))
    implementation(project(":blacknet-time"))
    implementation(libs.kotlin.stdlib)
    implementation(libs.ktor.network)
    implementation(libs.ktor.cio)
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
    implementation(libs.microutils.logging)
    implementation(libs.guava)
    implementation(libs.weupnp)
    implementation(files("buildSrc/libs/leveldbjni-all-1.18.3.jar"))
    testImplementation(libs.kotlin.testng) {
        exclude("aopalliance", "aopalliance")
        exclude("junit", "junit")
    }
}

application {
    mainClass.set("ninja.blacknet.Main")
}

distributions {
    main {
        contents {
            from("LICENSE.txt")
            from("3RD-PARTY-LICENSES.txt")
        }
    }
}

val compileJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "11"
}

val compileTestJava by tasks.existing(JavaCompile::class) {
    targetCompatibility = "11"
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_11)
        freeCompilerArgs.add("-Xjvm-default=all")
    }
}

val compileTestKotlin by tasks.existing(KotlinCompile::class) {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_11)
    }
}

val downloadLicenses by tasks.existing(DownloadLicenses::class) {
    doFirst {
        val konfigurations = configurations.toTypedArray()
        configurations.create("xonfigurations").extendsFrom(*konfigurations)
    }
    licenses = mapOf(
        libs.weupnp to LicenseMetadata("GNU LESSER GENERAL PUBLIC LICENSE 2.1", "https://www.gnu.org/licenses/old-licenses/lgpl-2.1.en.html"),
        libs.bouncycastle to LicenseMetadata("MIT License", "https://opensource.org/licenses/MIT")
    )
    dependencyConfiguration = "xonfigurations"
}

val jar by tasks.existing(Jar::class) {
    manifest {
        dirtyDescribeGit(buildDir.getParentFile())?.let { revision ->
            attributes("Build-Revision" to revision)
        }
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

val test by tasks.existing(Test::class) {
    useTestNG()
}

val wrapper by tasks.existing(Wrapper::class) {
    gradleVersion = "8.1"
    distributionType = Wrapper.DistributionType.BIN
    distributionSha256Sum = "a62c5f99585dd9e1f95dab7b9415a0e698fa9dd1e6c38537faa81ac078f4d23e"
}

val defaultSystemProperties: Map<String, Any> = mapOf(
    // Output hexadecimal values with lower case letters
    // "ninja.blacknet.codec.base.hex.lowercase" to true,
    // Indent JSON returned by RPC API
    // "ninja.blacknet.serialization.json.indented" to true,
    // Regression testing mode
    // "ninja.blacknet.regtest" to true,
)
