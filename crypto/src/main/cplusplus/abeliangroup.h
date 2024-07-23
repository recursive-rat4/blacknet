/*
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

#ifndef BLACKNET_CRYPTO_ABELIANGROUP_H
#define BLACKNET_CRYPTO_ABELIANGROUP_H

#include <algorithm>

// Speeding up the computations on an elliptic curve using addition-subtraction chains
// ADDSUBCHAIN-D
// http://www.numdam.org/item/ITA_1990__24_6_531_0/

namespace abeliangroup {

template<typename AG>
constexpr AG multiply(const AG& e, const typename AG::Scalar& s) {
    AG P(AG::LEFT_ADDITIVE_IDENTITY());
    AG Q(e);

    int QisQdouple = 0;
    int state = 0;

    auto updateQ = [&Q, &QisQdouple]() {
        for (int i = 0; i < QisQdouple; ++i) {
            Q = Q.douple();
        }
        QisQdouple = 0;
    };

    std::ranges::for_each(s.bitsBegin(), s.bitsEnd(), [&](bool bit) {
        switch(state){
            case 0:
                if(bit) {
                    state = 1;
                } else {
                    QisQdouple += 1;
                }
                break;
            case 1:
                // Q only needs to be updated in case P gets updated
                updateQ();
                if(bit) {
                    P = P - Q;
                    QisQdouple += 2;
                    state = 11;
                } else {
                    P = P + Q;
                    QisQdouple += 2;
                    state = 0;
                }
                break;
            case 11:
                if(bit) {
                    QisQdouple += 1;
                } else {
                    state = 1;
                }
                break;
        }
    });

    if(state!=0){
        // Q only needs to be updated in case P gets updated
        updateQ();
        P = P + Q;
    }

    return P;
}

}

#endif
