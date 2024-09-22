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
#include "bitint.h"
#include "primefield.h"
#include "weierstrassgroupaffine.h"
#include "weierstrassgroupjacobian.h"
#include "weierstrassgroupprojective.h"

namespace blacknet::crypto {

/*
 * The Pasta Curves for Halo 2 and Beyond
 * Daira Hopwood
 * November 23, 2020
 * https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/
 */

struct PallasFieldParams {
    constexpr static const std::size_t BITS = 255;
    constexpr static const UInt256 M = UInt256("40000000000000000000000000000000224698fc094cf91b992d30ed00000001");
    constexpr static const UInt256 R2 = UInt256("096d41af7b9cb7147797a99bc3c95d18d7d30dbd8b0de0e78c78ecb30000000f");
    constexpr static const UInt256 R3 = UInt256("2ae309222d2d9910df8d1014353fd42cf6a68f3b6ac5b1d1f185a5993a9e10f9");
    constexpr static const typename UInt256::L RN = UInt256::L(0x992d30ecffffffff);
    constexpr static const UInt256 TWO_INVERTED = UInt256("2000000000000000000000000000000011234c7e04a67c8dcc96987680000001");
    constexpr static const BitInt<254> P_MINUS_1_HALVED{"2000000000000000000000000000000011234c7e04a67c8dcc96987680000000"};
    constexpr static const BitInt<223> Q{"0000000040000000000000000000000000000000224698fc094cf91b992d30ed"};
    constexpr static const UInt256 S = UInt256(32);
    constexpr static const BitInt<222> Q_PLUS_1_HALVED{"000000002000000000000000000000000000000011234c7e04a67c8dcc969877"};
    constexpr static const bool has_sparse_modulus = true;

    constexpr static UInt256 reduce(const UInt512& x) {
        // Montgomery reduction
        UInt512 tt(x);
        typename UInt256::LL c = 0;
        for (std::size_t i = 0; i < UInt256::LIMBS(); ++i) {
            typename UInt256::LL ll = 0;
            typename UInt256::L l = tt.limbs[i] * RN;
            for (std::size_t j = 0; j < UInt256::LIMBS(); ++j) {
                ll += UInt256::LL(l) * UInt256::LL(M.limbs[j]) + UInt256::LL(tt.limbs[i + j]);
                tt.limbs[i + j] = ll;
                ll >>= sizeof(typename UInt256::L) * 8;
            }
            c += UInt256::LL(tt.limbs[i + UInt256::LIMBS()]) + ll;
            tt.limbs[i + UInt256::LIMBS()] = c;
            c >>= sizeof(typename UInt256::L) * 8;
        }
        UInt256 t{tt.limbs[7], tt.limbs[6], tt.limbs[5], tt.limbs[4]};
        if (t >= M)
            t -= M;
        return t;
    }

    constexpr static UInt256 toForm(const UInt256& n) {
        return reduce(n * R2);
    }
    constexpr static UInt256 fromForm(const UInt256& n) {
        return reduce(UInt512(0, 0, 0, 0, n.limbs[3], n.limbs[2], n.limbs[1], n.limbs[0]));
    }
};

typedef PrimeField<PallasFieldParams> PallasField;

struct VestaFieldParams {
    constexpr static const std::size_t BITS = 255;
    constexpr static const UInt256 M = UInt256("40000000000000000000000000000000224698fc0994a8dd8c46eb2100000001");
    constexpr static const UInt256 R2 = UInt256("096d41af7ccfdaa97fae231004ccf59067bb433d891a16e3fc9678ff0000000f");
    constexpr static const UInt256 R3 = UInt256("07dd97a06e6792c888fececb8e15cb63e13bda50dba41326008b421c249dae4c");
    constexpr static const typename UInt256::L RN = UInt256::L(0x8c46eb20ffffffff);
    constexpr static const UInt256 TWO_INVERTED = UInt256("2000000000000000000000000000000011234c7e04ca546ec623759080000001");
    constexpr static const BitInt<254> P_MINUS_1_HALVED{"2000000000000000000000000000000011234c7e04ca546ec623759080000000"};
    constexpr static const BitInt<223> Q{"0000000040000000000000000000000000000000224698fc0994a8dd8c46eb21"};
    constexpr static const UInt256 S = UInt256(32);
    constexpr static const BitInt<222> Q_PLUS_1_HALVED{"000000002000000000000000000000000000000011234c7e04ca546ec6237591"};
    constexpr static const bool has_sparse_modulus = true;

    constexpr static UInt256 reduce(const UInt512& x) {
        // Montgomery reduction
        UInt512 tt(x);
        typename UInt256::LL c = 0;
        for (std::size_t i = 0; i < UInt256::LIMBS(); ++i) {
            typename UInt256::LL ll = 0;
            typename UInt256::L l = tt.limbs[i] * RN;
            for (std::size_t j = 0; j < UInt256::LIMBS(); ++j) {
                ll += UInt256::LL(l) * UInt256::LL(M.limbs[j]) + UInt256::LL(tt.limbs[i + j]);
                tt.limbs[i + j] = ll;
                ll >>= sizeof(typename UInt256::L) * 8;
            }
            c += UInt256::LL(tt.limbs[i + UInt256::LIMBS()]) + ll;
            tt.limbs[i + UInt256::LIMBS()] = c;
            c >>= sizeof(typename UInt256::L) * 8;
        }
        UInt256 t{tt.limbs[7], tt.limbs[6], tt.limbs[5], tt.limbs[4]};
        if (t >= M)
            t -= M;
        return t;
    }

    constexpr static UInt256 toForm(const UInt256& n) {
        return reduce(n * R2);
    }
    constexpr static UInt256 fromForm(const UInt256& n) {
        return reduce(UInt512(0, 0, 0, 0, n.limbs[3], n.limbs[2], n.limbs[1], n.limbs[0]));
    }
};

typedef PrimeField<VestaFieldParams> VestaField;

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

}

#endif
