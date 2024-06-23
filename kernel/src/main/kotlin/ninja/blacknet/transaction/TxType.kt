/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.KSerializer

enum class TxType(val type: UByte) {
    Transfer(0u),
    Burn(1u),
    Lease(2u),
    CancelLease(3u),
    BApp(4u),
    CreateHTLC(5u),
    UnlockHTLC(6u),
    RefundHTLC(7u),
    SpendHTLC(8u),
    CreateMultisig(9u),
    SpendMultisig(10u),
    WithdrawFromLease(11u),
    ClaimHTLC(12u),
    //TODO Dispel(13u),
    Batch(16u),
    // Genesis(125u),
    Generated(254u),
    ;

    companion object {
        fun <T : TxData> getSerializer(type: UByte): KSerializer<T> {
            @Suppress("UNCHECKED_CAST")
            return when (type) {
                Transfer.type -> ninja.blacknet.transaction.Transfer.serializer()
                Burn.type -> ninja.blacknet.transaction.Burn.serializer()
                Lease.type -> ninja.blacknet.transaction.Lease.serializer()
                CancelLease.type -> ninja.blacknet.transaction.CancelLease.serializer()
                BApp.type -> ninja.blacknet.transaction.BApp.serializer()
                CreateHTLC.type -> ninja.blacknet.transaction.CreateHTLC.serializer()
                UnlockHTLC.type -> throw RuntimeException("Obsolete tx type UnlockHTLC")
                RefundHTLC.type -> ninja.blacknet.transaction.RefundHTLC.serializer()
                SpendHTLC.type -> throw RuntimeException("Obsolete tx type SpendHTLC")
                CreateMultisig.type -> ninja.blacknet.transaction.CreateMultisig.serializer()
                SpendMultisig.type -> ninja.blacknet.transaction.SpendMultisig.serializer()
                WithdrawFromLease.type -> ninja.blacknet.transaction.WithdrawFromLease.serializer()
                ClaimHTLC.type -> ninja.blacknet.transaction.ClaimHTLC.serializer()
                //Dispel.type -> ninja.blacknet.transaction.Dispel.serializer()
                Batch.type -> ninja.blacknet.transaction.Batch.serializer()
                Generated.type -> throw RuntimeException("Generated as individual tx")
                else -> throw RuntimeException("Unknown transaction type $type")
            } as KSerializer<T>
        }
    }
}
