/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.util.Random
import org.openjdk.jmh.annotations.Benchmark
import org.openjdk.jmh.annotations.Scope
import org.openjdk.jmh.annotations.State

@State(Scope.Benchmark)
class CommitmentBenchmark {
    private val seed = 20240508L
    private val random = Random(seed)

    private var g = PallasGroup.random(random)
    private var h = PallasGroup.random(random)
    private var pc = PedersenCommitment(g, h)
    private var m = VestaField.random(random)
    private var r = VestaField.random(random)

    @Benchmark
    fun pedersen() = pc.commit(m, r)
}
