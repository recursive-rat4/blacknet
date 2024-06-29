/*
 * Copyright (c) 2024 Pavel Vasin
 * Copyright (c) 2024 Xerxes Rånby
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

#ifndef BLACKNET_CRYPTO_SEMIGROUP_H
#define BLACKNET_CRYPTO_SEMIGROUP_H

#include <algorithm>
// Speeding up the computations on an elliptic curve using addition-subtraction chains
// ADDSUBCHAIN-A
// http://www.numdam.org/item/ITA_1990__24_6_531_0/

template<typename SG>
constexpr SG multiply(const SG& e, const typename SG::Scalar& s) {
    SG P(SG::LEFT_ADDITIVE_IDENTITY());
    SG Q(e);

    int state = 0;
    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        switch(state){
            case 0:
                if(bit) {
                    state = 1;
                } else {
                    Q = Q.douple();
                }
                break;
            case 1:
                if(bit) {
                    P = P - Q;
                    Q = Q.douple();
                    Q = Q.douple();
                    state = 11;
                } else {
                    P = P + Q;
                    Q = Q.douple();
                    Q = Q.douple();
                    state = 0;
                }
                break;
            case 11:
                if(bit) {
                    Q = Q.douple();
                } else {
                    P = P + Q;
                    Q = Q.douple();
                    state = 0;
                }
                break;
        }
    });

    if(state==1||state==11){
        P = P + Q;
    }

    return P;
}

template<typename SG>
constexpr SG power(const SG& e, const typename SG::Scalar& s) {
    // Square-and-multiply method
    SG r(SG::LEFT_MULTIPLICATIVE_IDENTITY());
    SG t(e);
    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        if (bit)
            r *= t;
        t = t.square();
    });
    return r;
}

#endif
