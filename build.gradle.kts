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
import org.eclipse.jgit.api.Git
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import java.net.URI

group = "ninja.blacknet"
version = "0.2.6"

buildscript {
    dependencies {
        "classpath"("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.14.2")
        "classpath"("org.eclipse.jgit:org.eclipse.jgit:5.7.0.202003110725-r")
    }
}

plugins {
    kotlin("jvm") version Versions.kotlin
    kotlin("plugin.serialization") version Versions.kotlin
    application
    distribution
    id("com.github.hierynomus.license-report") version "0.15.0"
}

apply<kotlinx.atomicfu.plugin.gradle.AtomicFUGradlePlugin>()

repositories {
    jcenter()
    maven { url = URI("https://dl.bintray.com/kotlin/ktor") }
    maven { url = URI("https://dl.bintray.com/kotlin/kotlinx") }
    maven { url = URI("https://dl.bintray.com/ethereum/maven") }
}

dependencies {
    "implementation"("org.jetbrains.kotlin:kotlin-stdlib-jdk8:${Versions.kotlin}")
    "implementation"("io.ktor:ktor-network:${Versions.ktor}")
    "implementation"("io.ktor:ktor-server-cio:${Versions.ktor}")
    "implementation"("io.ktor:ktor-websockets:${Versions.ktor}")
    "implementation"("org.jetbrains.kotlinx:kotlinx-coroutines-core-common:${Versions.coroutines}")
    "implementation"("org.jetbrains.kotlinx:kotlinx-coroutines-debug:${Versions.coroutines}")
    "implementation"("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:${Versions.coroutines}")
    "implementation"("org.jetbrains.kotlinx:kotlinx-serialization-runtime:${Versions.serialization}")
    "implementation"("net.i2p.crypto:eddsa:0.3.0")
    "implementation"("com.rfksystems:blake2b:1.0.0")
    "implementation"("org.iq80.leveldb:leveldb:0.12")
    "implementation"("org.slf4j:slf4j-jdk14:1.7.30")
    "implementation"("io.github.microutils:kotlin-logging:1.7.9")
    "implementation"("org.briarproject:jtorctl:0.3")
    "implementation"("com.google.guava:guava:28.2-jre")
    "implementation"("org.bitlet:weupnp:${Versions.weupnp}")
    "implementation"("org.bouncycastle:bcprov-jdk15on:${Versions.bouncycastle}")
    "implementation"("org.ethereum:leveldbjni-all:${Versions.leveldbjni}")
    "testImplementation"("org.testng:testng:7.1.1") {
        exclude("junit", "junit")
    }
}

application {
    mainClassName = "ninja.blacknet.Main"
}

distributions {
    main {
        contents {
            from("LICENSE.txt")
            from("3RD-PARTY-LICENSES.txt")
        }
    }
}

val compileKotlin by tasks.existing(KotlinCompile::class) {
    kotlinOptions {
        jvmTarget = "1.8"
        freeCompilerArgs += listOf(
                "-Xinline-classes",
                "-Xuse-experimental=kotlin.ExperimentalUnsignedTypes"
        )
    }
}

val dirtyDescribeGit by tasks.registering {
    doLast {
        val git = Git.open(buildDir.getParentFile())
        val describtion = git.describe().call()
        val status = git.status().call()
        project.extra["revision"] =
            if (status.hasUncommittedChanges())
                "$describtion-dirty"
            else
                describtion
    }
}

val createVersionTxt by tasks.registering {
    dependsOn(processResources)
    dependsOn(dirtyDescribeGit)
    doLast {
        File("$buildDir/resources/main/revision.txt").writeText("${project.extra["revision"]}")
        File("$buildDir/resources/main/version.txt").writeText(project.version.toString())
        File("$buildDir/resources/main/ktor_version.txt").writeText(Versions.ktor)
    }
}

val downloadLicenses by tasks.existing(DownloadLicenses::class) {
    doFirst {
        val konfigurations = configurations.toTypedArray()
        configurations.create("xonfigurations").extendsFrom(*konfigurations)
    }
    licenses = mapOf(
        "org.bitlet:weupnp:${Versions.weupnp}" to LicenseMetadata("GNU LESSER GENERAL PUBLIC LICENSE 2.1", "https://www.gnu.org/licenses/old-licenses/lgpl-2.1.en.html"),
        "org.bouncycastle:bcprov-jdk15on:${Versions.bouncycastle}" to LicenseMetadata("MIT License", "https://opensource.org/licenses/MIT"),
        "org.ethereum:leveldbjni-all:${Versions.leveldbjni}" to LicenseMetadata("BSD-3-clause", "https://opensource.org/licenses/BSD-3-Clause")
    )
    dependencyConfiguration = "xonfigurations"
}

val classes by tasks.existing {
    dependsOn(createVersionTxt)
}

val processResources by tasks.existing(ProcessResources::class)

val test by tasks.existing(Test::class) {
    useTestNG()
}
