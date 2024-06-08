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
#include "weierstrassgroupaffine.h"
#include "weierstrassgroupprojective.h"

/*
 * The Pasta Curves for Halo 2 and Beyond
 * Daira Hopwood
 * November 23, 2020
 * https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/
 */

typedef PrimeField<
    255,
    UInt256(0x4000000000000000, 0x0000000000000000, 0x224698fc094cf91b, 0x992d30ed00000001),
    UInt512(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000003,
            0xffffffffffffffff, 0xfffffffffffffffd, 0xdb96703f6b306e46, 0x6d2cf12ffffffff1),
    UInt256(0x4000000000000000, 0x0000000000000000, 0x224698fc094cf91b, 0x992d30ecffffffff),
    UInt256(0x2000000000000000, 0x0000000000000000, 0x11234c7e04a67c8d, 0xcc96987680000000),
    UInt256(0x0000000040000000, 0x0000000000000000, 0x00000000224698fc, 0x094cf91b992d30ed),
    UInt256(32),
    UInt256(0x0000000020000000, 0x0000000000000000, 0x0000000011234c7e, 0x04a67c8dcc969877)
> PallasField;

typedef PrimeField<
    255,
    UInt256(0x4000000000000000, 0x0000000000000000, 0x224698fc0994a8dd, 0x8c46eb2100000001),
    UInt512(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000003,
            0xffffffffffffffff, 0xfffffffffffffffd, 0xdb96703f66b57227, 0x3b914deffffffff1),
    UInt256(0x4000000000000000, 0x0000000000000000, 0x224698fc0994a8dd, 0x8c46eb20ffffffff),
    UInt256(0x2000000000000000, 0x0000000000000000, 0x11234c7e04ca546e, 0xc623759080000000),
    UInt256(0x0000000040000000, 0x0000000000000000, 0x00000000224698fc, 0x0994a8dd8c46eb21),
    UInt256(32),
    UInt256(0x0000000020000000, 0x0000000000000000, 0x0000000011234c7e, 0x04ca546ec6237591)
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

#endif
