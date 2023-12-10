/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.KSerializer
import kotlinx.serialization.modules.SerializersModule
import kotlinx.serialization.modules.SerializersModuleBuilder
import ninja.blacknet.contract.*
import ninja.blacknet.crypto.*
import kotlin.reflect.KClass

fun <T : Any> serializersModuleOf(kClass: KClass<out ContextualSerializer<T>>, serializer: KSerializer<T>): SerializersModule {
    return SerializersModule { contextual(kClass, serializer) }
}

fun <T : Any> SerializersModuleBuilder.contextual(kClass: KClass<out ContextualSerializer<T>>, serializer: KSerializer<T>) {
    @Suppress("UNCHECKED_CAST")
    return contextual(kClass as KClass<T>, serializer)
}

val binaryModule: SerializersModule = SerializersModule {
    contextual(BigIntegerSerializer::class, BigIntegerAsBinarySerializer)
    contextual(ByteArraySerializer::class, ByteArrayAsBinarySerializer)
    contextual(HashSerializer::class, HashAsBinarySerializer)
    contextual(HashTimeLockContractIdSerializer::class, HashTimeLockContractIdAsBinarySerializer)
    contextual(MultiSignatureLockContractIdSerializer::class, MultiSignatureLockContractIdAsBinarySerializer)
    contextual(PrivateKeySerializer::class, PrivateKeyAsBinarySerializer)
    contextual(PublicKeySerializer::class, PublicKeyAsBinarySerializer)
    contextual(SignatureSerializer::class, SignatureAsBinarySerializer)
}

val textModule: SerializersModule = SerializersModule {
    contextual(BigIntegerSerializer::class, BigIntegerAsStringSerializer)
    contextual(ByteArraySerializer::class, ByteArrayAsStringSerializer)
    contextual(HashSerializer::class, HashAsStringSerializer)
    contextual(HashTimeLockContractIdSerializer::class, HashTimeLockContractIdAsStringSerializer)
    contextual(MultiSignatureLockContractIdSerializer::class, MultiSignatureLockContractIdAsStringSerializer)
    contextual(PrivateKeySerializer::class, PrivateKeyAsStringSerializer)
    contextual(PublicKeySerializer::class, PublicKeyAsStringSerializer)
    contextual(SignatureSerializer::class, SignatureAsStringSerializer)
}
