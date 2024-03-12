/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlin.random.Random
import kotlin.random.nextUInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PaymentId
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.transaction.Transfer
import ninja.blacknet.transaction.TxType
import org.openjdk.jmh.annotations.Benchmark
import org.openjdk.jmh.annotations.Scope
import org.openjdk.jmh.annotations.State

@State(Scope.Benchmark)
class BlockBenchmark {
    private val seed = 20240128
    private val random = Random(seed)

    private var empty = makeBlock(0)
    private var emptyBytes = binaryFormat.encodeToByteArray(Block.serializer(), empty)
    private var thousandTx = makeBlock(1000)
    private var thousandTxBytes = binaryFormat.encodeToByteArray(Block.serializer(), thousandTx)

    @Benchmark
    fun serializeEmpty() = binaryFormat.encodeToByteArray(Block.serializer(), empty)

    @Benchmark
    fun deserializeEmpty() = binaryFormat.decodeFromByteArray(Block.serializer(), emptyBytes)

    @Benchmark
    fun serializeThousandTx() = binaryFormat.encodeToByteArray(Block.serializer(), thousandTx)

    @Benchmark
    fun deserializeThousandTx() = binaryFormat.decodeFromByteArray(Block.serializer(), thousandTxBytes)

    @Benchmark
    fun sizeEmpty() = binaryFormat.computeSize(Block.serializer(), empty)

    @Benchmark
    fun sizeThousandTx() = binaryFormat.computeSize(Block.serializer(), thousandTx)

    private fun makeBlock(tx: Int) = Block(
        random.nextUInt(),
        Hash(random.nextBytes(Hash.SIZE_BYTES)),
        random.nextLong(),
        PublicKey(random.nextBytes(PublicKey.SIZE_BYTES)),
        Hash(random.nextBytes(Hash.SIZE_BYTES)),
        random.nextBytes(SignatureSerializer.SIZE_BYTES),
        ArrayList(),
    ).apply {
        repeat(tx) {
            transactions.add(
                binaryFormat.encodeToByteArray(Transaction.serializer(),
                    Transaction(
                        random.nextBytes(SignatureSerializer.SIZE_BYTES),
                        PublicKey(random.nextBytes(PublicKey.SIZE_BYTES)),
                        random.nextInt(),
                        Hash(random.nextBytes(Hash.SIZE_BYTES)),
                        random.nextLong(),
                        TxType.Transfer.type,
                        binaryFormat.encodeToByteArray(Transfer.serializer(),
                            Transfer(
                                random.nextLong(),
                                PublicKey(random.nextBytes(PublicKey.SIZE_BYTES)),
                                PaymentId.EMPTY,
                            )
                        )
                    )
                )
            )
        }
    }
}
