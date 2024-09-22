/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef BLACKNET_CRYPTO_PASTACURVES_H
#define BLACKNET_CRYPTO_PASTACURVES_H

#include "bigint.h"
#include "primefield.h"
#include "primefieldbarrett.h"
#include "weierstrassgroupaffine.h"
#include "weierstrassgroupjacobian.h"
#include "weierstrassgroupprojective.h"

/*
 * The Pasta Curves for Halo 2 and Beyond
 * Daira Hopwood
 * November 23, 2020
 * https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/
 */

typedef PrimeFieldBarrett<
    255,
    UInt256("40000000000000000000000000000000224698fc094cf91b992d30ed00000001"),
    UInt512("0000000000000000000000000000000000000000000000000000000000000003"
            "fffffffffffffffffffffffffffffffddb96703f6b306e466d2cf12ffffffff1"),
    UInt256("2000000000000000000000000000000011234c7e04a67c8dcc96987680000000"),
    UInt256("0000000040000000000000000000000000000000224698fc094cf91b992d30ed"),
    UInt256(32),
    UInt256("000000002000000000000000000000000000000011234c7e04a67c8dcc969877")
> PallasFieldBarrett;

typedef PrimeField<
    255,
    UInt256("40000000000000000000000000000000224698fc094cf91b992d30ed00000001"),
    UInt256("096d41af7b9cb7147797a99bc3c95d18d7d30dbd8b0de0e78c78ecb30000000f"),
    UInt256("2ae309222d2d9910df8d1014353fd42cf6a68f3b6ac5b1d1f185a5993a9e10f9"),
    UInt256::L(0x992d30ecffffffff),
    UInt256("2000000000000000000000000000000011234c7e04a67c8dcc96987680000000"),
    UInt256("0000000040000000000000000000000000000000224698fc094cf91b992d30ed"),
    UInt256(32),
    UInt256("000000002000000000000000000000000000000011234c7e04a67c8dcc969877")
> PallasField;

typedef PrimeFieldBarrett<
    255,
    UInt256("40000000000000000000000000000000224698fc0994a8dd8c46eb2100000001"),
    UInt512("0000000000000000000000000000000000000000000000000000000000000003"
            "fffffffffffffffffffffffffffffffddb96703f66b572273b914deffffffff1"),
    UInt256("2000000000000000000000000000000011234c7e04ca546ec623759080000000"),
    UInt256("0000000040000000000000000000000000000000224698fc0994a8dd8c46eb21"),
    UInt256(32),
    UInt256("000000002000000000000000000000000000000011234c7e04ca546ec6237591")
> VestaFieldBarrett;

typedef PrimeField<
    255,
    UInt256("40000000000000000000000000000000224698fc0994a8dd8c46eb2100000001"),
    UInt256("096d41af7ccfdaa97fae231004ccf59067bb433d891a16e3fc9678ff0000000f"),
    UInt256("07dd97a06e6792c888fececb8e15cb63e13bda50dba41326008b421c249dae4c"),
    UInt256::L(0x8c46eb20ffffffff),
    UInt256("2000000000000000000000000000000011234c7e04ca546ec623759080000000"),
    UInt256("0000000040000000000000000000000000000000224698fc0994a8dd8c46eb21"),
    UInt256(32),
    UInt256("000000002000000000000000000000000000000011234c7e04ca546ec6237591")
> VestaField;

typedef WeierstrassGroupAffine<
    PallasField,
    VestaField,
    PallasField(0),
    PallasField(5)
> PallasGroupAffine;

typedef WeierstrassGroupAffine<
    VestaField,
    PallasField,
    VestaField(0),
    VestaField(5)
> VestaGroupAffine;

typedef WeierstrassGroupProjective<
    PallasField,
    VestaField,
    PallasField(0),
    PallasField(5)
> PallasGroupProjective;

typedef WeierstrassGroupProjective<
    VestaField,
    PallasField,
    VestaField(0),
    VestaField(5)
> VestaGroupProjective;

typedef WeierstrassGroupJacobian<
    PallasField,
    VestaField,
    PallasField(0),
    PallasField(5)
> PallasGroupJacobian;

typedef WeierstrassGroupJacobian<
    VestaField,
    PallasField,
    VestaField(0),
    VestaField(5)
> VestaGroupJacobian;

#endif
