/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.KSerializer

enum class TxType {
    Transfer,
    Burn,
    Lease,
    CancelLease,
    Bundle,
    CreateHTLC,
    UnlockHTLC,
    RefundHTLC,
    SpendHTLC,
    CreateMultisig,
    SpendMultisig,
    ;

    fun getType(): Byte = ordinal.toByte()

    companion object {
        fun getSerializer(type: Byte): KSerializer<out TxData>? {
            return when (type) {
                Transfer.ordinal.toByte() -> ninja.blacknet.core.Transfer.serializer()
                Burn.ordinal.toByte() -> ninja.blacknet.core.Burn.serializer()
                Lease.ordinal.toByte() -> ninja.blacknet.core.Lease.serializer()
                CancelLease.ordinal.toByte() -> ninja.blacknet.core.CancelLease.serializer()
                Bundle.ordinal.toByte() -> ninja.blacknet.core.Bundle.serializer()
                CreateHTLC.ordinal.toByte() -> ninja.blacknet.core.CreateHTLC.serializer()
                UnlockHTLC.ordinal.toByte() -> ninja.blacknet.core.UnlockHTLC.serializer()
                RefundHTLC.ordinal.toByte() -> ninja.blacknet.core.RefundHTLC.serializer()
                SpendHTLC.ordinal.toByte() -> ninja.blacknet.core.SpendHTLC.serializer()
                CreateMultisig.ordinal.toByte() -> ninja.blacknet.core.CreateMultisig.serializer()
                SpendMultisig.ordinal.toByte() -> ninja.blacknet.core.SpendMultisig.serializer()
                else -> null
            }
        }
    }
}
