/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

import org.gradle.api.Project

private val description = "Opt out of opt in"

// use OW2 ASM?
private val compilerArgs = mapOf(
    "kotlinx-coroutines-core" to "kotlinx.coroutines.DelicateCoroutinesApi",
    "kotlinx-coroutines-debug" to "kotlinx.coroutines.ExperimentalCoroutinesApi",
    "kotlinx-serialization-core" to "kotlinx.serialization.ExperimentalSerializationApi",
    "ktor-io" to "io.ktor.utils.io.core.ExperimentalIoApi",
).asSequence().associate { (name, annotation) ->
    name to "-opt-in=$annotation"
}

fun optOut(project: Project): Collection<String> {
    val result = HashSet<String>()
    project.configurations.forEach { configuration ->
        configuration.dependencies.forEach { dependency ->
            compilerArgs.get(dependency.name)?.let { compilerArg ->
                result.add(compilerArg)
            }
        }
    }
    return result
}
