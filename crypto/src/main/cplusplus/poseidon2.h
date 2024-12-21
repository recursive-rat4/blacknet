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

#ifndef BLACKNET_CRYPTO_POSEIDON2_H
#define BLACKNET_CRYPTO_POSEIDON2_H

#include <array>
#include <concepts>
#include <ranges>
#include <vector>

/*
 * Poseidon2: A Faster Version of the Poseidon Hash Function
 * Lorenzo Grassi, Dmitry Khovratovich, Markus Schofnegger
 * February 8, 2024
 * https://eprint.iacr.org/2023/323
 */

namespace poseidon2 {

namespace {

template<typename Params>
constexpr void m4(std::array<typename Params::F, Params::t>& x) {
    using F = Params::F;
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T >> 2; ++i) {
        std::size_t j = i << 2;
        std::array<F, 8> t;
        t[0] = x[j] + x[j + 1];
        t[1] = x[j + 2] + x[j + 3];
        t[2] = x[j + 1].douple() + t[1];
        t[3] = x[j + 3].douple() + t[0];
        t[4] = t[1].douple().douple() + t[3];
        t[5] = t[0].douple().douple() + t[2];
        t[6] = t[3] + t[5];
        t[7] = t[2] + t[4];
        x[j] = t[6];
        x[j + 1] = t[5];
        x[j + 2] = t[7];
        x[j + 3] = t[4];
    }
}

template<typename Params>
constexpr void external(std::array<typename Params::F, Params::t>& x) {
    using F = Params::F;
    constexpr std::size_t T = Params::t;

    if constexpr (T == 2) {
        F s(x[0] + x[1]);
        x[0] += s;
        x[1] += s;
    } else if constexpr (T == 3) {
        F s(x[0] + x[1] + x[2]);
        x[0] += s;
        x[1] += s;
        x[2] += s;
    } else if constexpr (T == 4) {
        m4<Params>(x);
    } else if constexpr (T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
        m4<Params>(x);
        std::array<F, 4> s;
        for (std::size_t i = 0; i < 4; ++i) {
            s[i] = x[i];
            for (std::size_t j = 1; j < T >> 2; ++j)
                s[i] += x[(j << 2) + i];
        }
        for (std::size_t i = 0; i < T; ++i)
            x[i] += s[i & 3];
    } else {
        static_assert(false);
    }
}

template<typename Params>
constexpr void internal(std::array<typename Params::F, Params::t>& x) {
    using F = Params::F;
    constexpr std::size_t T = Params::t;

    if constexpr (T == 2) {
        F s(x[0] + x[1]);
        x[0] += s;
        x[1] = x[1].douple() + s;
    } else if constexpr (T == 3) {
        F s(x[0] + x[1] + x[2]);
        x[0] += s;
        x[1] += s;
        x[2] = x[2].douple() + s;
    } else if constexpr (T == 4 || T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
        F s(x[0]);
        for (std::size_t i = 1; i < T; ++i)
            s += x[i];
        for (std::size_t i = 0; i < T; ++i)
            x[i] = x[i] * Params::m[i] + s;
    } else {
        static_assert(false);
    }
}

template<typename Params>
constexpr void rcb(std::size_t round, std::array<typename Params::F, Params::t>& x) {
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T; ++i)
        x[i] += Params::rcb[round * T + i];
}

template<typename Params>
constexpr void rcp(std::size_t round, std::array<typename Params::F, Params::t>& x) {
    x[0] += Params::rcp[round];
}

template<typename Params>
constexpr void rce(std::size_t round, std::array<typename Params::F, Params::t>& x) {
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T; ++i)
        x[i] += Params::rce[round * T + i];
}

template<typename Params>
constexpr void sboxp(typename Params::F& x) {
    constexpr std::size_t A = Params::a;

    if constexpr (A == 3) {
        x *= x.square();
    } else if constexpr (A == 5) {
        x *= x.square().square();
    } else if constexpr (A == 17) {
        x *= x.square().square().square().square();
    } else {
        static_assert(false);
    }
}

template<typename Params>
constexpr void sbox(std::array<typename Params::F, Params::t>& x) {
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T; ++i)
        sboxp<Params>(x[i]);
}

}

template<typename Params>
constexpr void permute(std::array<typename Params::F, Params::t>& x) {
    external<Params>(x);

    for (std::size_t round = 0; round < Params::rb; ++round) {
        rcb<Params>(round, x);
        sbox<Params>(x);
        external<Params>(x);
    }

    for (std::size_t round = 0; round < Params::rp; ++round) {
        rcp<Params>(round, x);
        sboxp<Params>(x[0]);
        internal<Params>(x);
    }

    for (std::size_t round = 0; round < Params::re; ++round) {
        rce<Params>(round, x);
        sbox<Params>(x);
        external<Params>(x);
    }
}

namespace circuit {
template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr typename Circuit::LinearCombination sum(
    const std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    typename Circuit::LinearCombination lc;
    for (std::size_t i = 0; i < Params::t; ++i) {
        lc += y[i];
    }
    return lc;
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void m4(
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    using F = Params::F;
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T >> 2; ++i) {
        std::size_t j = i << 2;
        std::array<typename Circuit::LinearCombination, 8> t;
        t[0] = y[j] + y[j + 1];
        t[1] = y[j + 2] + y[j + 3];
        t[2] = y[j + 1] * F(2) + t[1];
        t[3] = y[j + 3] * F(2) + t[0];
        t[4] = t[1] * F(4) + t[3];
        t[5] = t[0] * F(4) + t[2];
        t[6] = t[3] + t[5];
        t[7] = t[2] + t[4];
        y[j] = std::move(t[6]);
        y[j + 1] = std::move(t[5]);
        y[j + 2] = std::move(t[7]);
        y[j + 3] = std::move(t[4]);
    }
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void external(
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    constexpr std::size_t T = Params::t;

    if constexpr (T == 2) {
        auto s = sum<Params, Circuit>(y);
        y[0] += s;
        y[1] += s;
    } else if constexpr (T == 3) {
        auto s = sum<Params, Circuit>(y);
        y[0] += s;
        y[1] += s;
        y[2] += s;
    } else if constexpr (T == 4) {
        m4<Params, Circuit>(y);
    } else if constexpr (T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
        m4<Params, Circuit>(y);
        std::array<typename Circuit::LinearCombination, 4> s;
        for (std::size_t i = 0; i < 4; ++i) {
            s[i] = y[i];
            for (std::size_t j = 1; j < T >> 2; ++j)
                s[i] += y[(j << 2) + i];
        }
        for (std::size_t i = 0; i < T; ++i)
            y[i] += s[i & 3];
    } else {
        static_assert(false);
    }
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void internal(
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    auto s = sum<Params, Circuit>(y);
    for (std::size_t i = 0; i < Params::t; ++i) {
        y[i] *= Params::m[i];
        y[i] += s;
    }
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void rcb(
    std::size_t round,
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T; ++i) {
        y[i] += Params::rcb[round * T + i];
    }
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void rcp(
    std::size_t round,
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    y[0] += Params::rcp[round];
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void rce(
    std::size_t round,
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T; ++i) {
        y[i] += Params::rce[round * T + i];
    }
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void sboxp(
    Circuit& circuit,
    typename Circuit::Variable& x,
    typename Circuit::LinearCombination& y
) {
    if constexpr (Params::a == 3) {
        if constexpr (circuit.degree() >= 3) {
            auto x3 = circuit.auxiliary();
            circuit(x3 == y * y * y);
            x = x3;
            y = x;
        } else {
            auto x2 = circuit.auxiliary();
            auto x3 = circuit.auxiliary();
            circuit(x2 == y * y);
            circuit(x3 == y * x2);
            x = x3;
            y = x;
        }
    } else if constexpr (Params::a == 5) {
        // Lessen constraints if degree >= 4
        auto x2 = circuit.auxiliary();
        auto x4 = circuit.auxiliary();
        auto x5 = circuit.auxiliary();
        circuit(x2 == y * y);
        circuit(x4 == x2 * x2);
        circuit(x5 == y * x4);
        x = x5;
        y = x;
    } else if constexpr (Params::a == 17) {
        // Lessen constraints if degree >= 4
        auto x2 = circuit.auxiliary();
        auto x4 = circuit.auxiliary();
        auto x8 = circuit.auxiliary();
        auto x16 = circuit.auxiliary();
        auto x17 = circuit.auxiliary();
        circuit(x2 == y * y);
        circuit(x4 == x2 * x2);
        circuit(x8 == x4 * x4);
        circuit(x16 == x8 * x8);
        circuit(x17 == y * x16);
        x = x17;
        y = x;
    } else {
        static_assert(false, "Not implemented");
    }
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void sbox(
    Circuit& circuit,
    std::array<typename Circuit::Variable, Params::t>& x,
    std::array<typename Circuit::LinearCombination, Params::t>& y
) {
    for (std::size_t i = 0; i < Params::t; ++i)
        sboxp<Params, Circuit>(circuit, x[i], y[i]);
}

template<typename Params, typename Circuit>
requires(std::same_as<typename Params::F, typename Circuit::R>)
constexpr void permute(
    Circuit& circuit,
    std::array<typename Circuit::Variable, Params::t>& x
) {
    auto scope = circuit.scope("Poseidon2::permute");

    std::array<typename Circuit::LinearCombination, Params::t> y;
    for (std::size_t i = 0; i < Params::t; ++i)
        y[i] = x[i];

    circuit::external<Params, Circuit>(y);

    for (std::size_t round = 0; round < Params::rb; ++round) {
        circuit::rcb<Params, Circuit>(round, y);
        circuit::sbox<Params, Circuit>(circuit, x, y);
        circuit::external<Params, Circuit>(y);
    }

    for (std::size_t round = 0; round < Params::rp; ++round) {
        circuit::rcp<Params, Circuit>(round, y);
        circuit::sboxp<Params, Circuit>(circuit, x[0], y[0]);
        circuit::internal<Params, Circuit>(y);
    }

    for (std::size_t round = 0; round < Params::re; ++round) {
        circuit::rce<Params, Circuit>(round, y);
        circuit::sbox<Params, Circuit>(circuit, x, y);
        circuit::external<Params, Circuit>(y);
    }

    for (std::size_t i = 0; i < Params::t; ++i) {
        auto v = circuit.auxiliary();
        circuit(v == y[i]);
        x[i] = v;
    }
}
}

namespace trace {
template<typename Params, std::size_t circuit>
constexpr void sboxp(typename Params::F& x, std::vector<typename Params::F>& trace) {
    constexpr std::size_t A = Params::a;

    if constexpr (A == 3) {
        if constexpr (circuit >= 3) {
            trace.push_back(
                x *= x.square()
            );
        } else {
            trace.push_back(
                x *= trace.emplace_back(
                    x.square())
            );
        }
    } else if constexpr (A == 5) {
        trace.push_back(
            x *= trace.emplace_back(trace.emplace_back(
                x.square()).square())
        );
    } else if constexpr (A == 17) {
        trace.push_back(
            x *= trace.emplace_back(trace.emplace_back(trace.emplace_back(trace.emplace_back(
                x.square()).square()).square()).square())
        );
    } else {
        static_assert(false, "Not implemented");
    }
}

template<typename Params, std::size_t circuit>
constexpr void sbox(std::array<typename Params::F, Params::t>& x, std::vector<typename Params::F>& trace) {
    constexpr std::size_t T = Params::t;

    for (std::size_t i = 0; i < T; ++i)
        sboxp<Params, circuit>(x[i], trace);
}

template<typename Params, std::size_t circuit>
constexpr void permute(std::array<typename Params::F, Params::t>& x, std::vector<typename Params::F>& trace) {
    external<Params>(x);

    for (std::size_t round = 0; round < Params::rb; ++round) {
        rcb<Params>(round, x);
        sbox<Params, circuit>(x, trace);
        external<Params>(x);
    }

    for (std::size_t round = 0; round < Params::rp; ++round) {
        rcp<Params>(round, x);
        sboxp<Params, circuit>(x[0], trace);
        internal<Params>(x);
    }

    for (std::size_t round = 0; round < Params::re; ++round) {
        rce<Params>(round, x);
        sbox<Params, circuit>(x, trace);
        external<Params>(x);
    }

    std::ranges::copy(x, std::back_inserter(trace));
}
}

}

#endif
