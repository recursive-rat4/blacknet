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

allprojects {
    group = "ninja.blacknet"
    version = "0.3-SNAPSHOT"
}

//UPSTREAM must be in subproject
buildscript {
    dependencies {
        classpath(libs.kotlin.atomicfu)
    }
}

plugins {
    alias(libs.plugins.kotlin.jvm) apply false //UPSTREAM must be in subproject
    distribution
    alias(libs.plugins.licenses)
}

repositories {
    mavenCentral()
}

val dependencies = arrayOf(
    project(":blacknet-daemon"),
)

distributions {
    main {
        contents {
            from("LICENSE.txt")
            from("3RD-PARTY-LICENSES.txt")
            dependencies.forEach {
                from(it.buildDir.resolve("install").resolve(it.name))
            }
        }
    }
}

val distTar by tasks.existing(Tar::class) {
    dependencies.forEach {
        dependsOn(it.tasks.installDist)
    }
}

val distZip by tasks.existing(Zip::class) {
    dependencies.forEach {
        dependsOn(it.tasks.installDist)
    }
}

val installDist by tasks.existing(Sync::class) {
    dependencies.forEach {
        dependsOn(it.tasks.installDist)
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

val wrapper by tasks.existing(Wrapper::class) {
    gradleVersion = "8.4"
    distributionType = Wrapper.DistributionType.BIN
    distributionSha256Sum = "3e1af3ae886920c3ac87f7a91f816c0c7c436f276a6eefdb3da152100fef72ae"
}
