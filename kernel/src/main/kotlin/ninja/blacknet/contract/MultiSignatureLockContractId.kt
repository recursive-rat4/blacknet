/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import java.util.Arrays
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.HashEncoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.bbf.BinaryDecoder
import ninja.blacknet.serialization.bbf.BinaryEncoder
import ninja.blacknet.serialization.config.ConfigDecoder
import ninja.blacknet.serialization.notSupportedFormatError

/**
 * An id of a multi-signature lock contract.
 */
@Serializable(MultiSignatureLockContractId.Companion::class)
class MultiSignatureLockContractId(
    val bytes: ByteArray
) : Comparable<MultiSignatureLockContractId> {
    override fun equals(other: Any?): Boolean {
        return (other is MultiSignatureLockContractId) && bytes.contentEquals(other.bytes)
    }

    override fun hashCode(): Int {
        return hashCode(serializer(), this)
    }

    override fun toString(): String {
        return Address.encode(Address.MULTISIG, bytes)
    }

    override fun compareTo(other: MultiSignatureLockContractId): Int {
        return Arrays.compareUnsigned(bytes, other.bytes)
    }

    companion object : KSerializer<MultiSignatureLockContractId> {
        /**
         * The number of bytes in a binary representation of the multi-signature lock contract id.
         */
        const val SIZE_BYTES = Hash.SIZE_BYTES

        //JSON override val descriptor: SerialDescriptor = ListSerialDescriptor("ninja.blacknet.contract.MultiSignatureLockContractId", Byte.serializer().descriptor)
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("ninja.blacknet.contract.MultiSignatureLockContractId", PrimitiveKind.STRING)

        override fun deserialize(decoder: Decoder) = MultiSignatureLockContractId(
            when (decoder) {
                is BinaryDecoder,
                    -> decoder.decodeFixedByteArray(SIZE_BYTES)
                is ConfigDecoder,
                is JsonDecoder,
                is RequestDecoder,
                    -> Address.decode(Address.MULTISIG, decoder.decodeString())
                else
                    -> throw notSupportedFormatError(decoder, this)
            }
        )

        override fun serialize(encoder: Encoder, value: MultiSignatureLockContractId) {
            when (encoder) {
                is BinaryEncoder,
                    -> encoder.encodeFixedByteArray(value.bytes)
                is HashEncoder,
                    -> encoder.encodeByteArray(value.bytes)
                is JsonEncoder,
                    -> encoder.encodeString(Address.encode(Address.MULTISIG, value.bytes))
                else
                    -> throw notSupportedFormatError(encoder, this)
            }
        }
    }
}
