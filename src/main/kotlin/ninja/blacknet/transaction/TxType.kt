/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

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
                Transfer.ordinal.toByte() -> ninja.blacknet.transaction.Transfer.serializer()
                Burn.ordinal.toByte() -> ninja.blacknet.transaction.Burn.serializer()
                Lease.ordinal.toByte() -> ninja.blacknet.transaction.Lease.serializer()
                CancelLease.ordinal.toByte() -> ninja.blacknet.transaction.CancelLease.serializer()
                Bundle.ordinal.toByte() -> ninja.blacknet.transaction.Bundle.serializer()
                CreateHTLC.ordinal.toByte() -> ninja.blacknet.transaction.CreateHTLC.serializer()
                UnlockHTLC.ordinal.toByte() -> ninja.blacknet.transaction.UnlockHTLC.serializer()
                RefundHTLC.ordinal.toByte() -> ninja.blacknet.transaction.RefundHTLC.serializer()
                SpendHTLC.ordinal.toByte() -> ninja.blacknet.transaction.SpendHTLC.serializer()
                CreateMultisig.ordinal.toByte() -> ninja.blacknet.transaction.CreateMultisig.serializer()
                SpendMultisig.ordinal.toByte() -> ninja.blacknet.transaction.SpendMultisig.serializer()
                else -> null
            }
        }
    }
}
