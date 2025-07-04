/*
 * Copyright (c) 2024-2025 Pavel Vasin
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
#include <vector>

namespace blacknet::crypto {

/*
 * Poseidon2: A Faster Version of the Poseidon Hash Function
 * Lorenzo Grassi, Dmitry Khovratovich, Markus Schofnegger
 * February 8, 2024
 * https://eprint.iacr.org/2023/323
 */

template<typename Params>
struct Poseidon2 {
    using F = Params::F;
    consteval static std::size_t width() { return Params::t; }
private:
    constexpr static std::size_t T = Params::t;

    constexpr static void m4(std::array<F, T>& x) {
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

    constexpr static void external(std::array<F, T>& x) {
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
            m4(x);
        } else if constexpr (T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
            m4(x);
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

    constexpr static void internal(std::array<F, T>& x) {
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

    constexpr static void rcb(std::size_t round, std::array<F, T>& x) {
        for (std::size_t i = 0; i < T; ++i)
            x[i] += Params::rcb[round * T + i];
    }

    constexpr static void rcp(std::size_t round, std::array<F, T>& x) {
        x[0] += Params::rcp[round];
    }

    constexpr static void rce(std::size_t round, std::array<F, T>& x) {
        for (std::size_t i = 0; i < T; ++i)
            x[i] += Params::rce[round * T + i];
    }

    constexpr static void sboxp(F& x) {
        if constexpr (Params::a == 3) {
            x *= x.square();
        } else if constexpr (Params::a == 5) {
            x *= x.square().square();
        } else if constexpr (Params::a == 17) {
            x *= x.square().square().square().square();
        } else {
            static_assert(false);
        }
    }

    constexpr static void sbox(std::array<F, T>& x) {
        for (std::size_t i = 0; i < T; ++i)
            sboxp(x[i]);
    }
public:
    constexpr static void permute(std::array<F, T>& x) {
        external(x);

        for (std::size_t round = 0; round < Params::rb; ++round) {
            rcb(round, x);
            sbox(x);
            external(x);
        }

        for (std::size_t round = 0; round < Params::rp; ++round) {
            rcp(round, x);
            sboxp(x[0]);
            internal(x);
        }

        for (std::size_t round = 0; round < Params::re; ++round) {
            rce(round, x);
            sbox(x);
            external(x);
        }
    }

template<typename Builder>
requires(std::same_as<typename Params::F, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
private:
    constexpr static LinearCombination sum(const std::array<LinearCombination, T>& x) {
        LinearCombination lc;
        for (std::size_t i = 0; i < T; ++i) {
            lc += x[i];
        }
        return lc;
    }

    constexpr static void m4(std::array<LinearCombination, T>& x) {
        for (std::size_t i = 0; i < T >> 2; ++i) {
            std::size_t j = i << 2;
            std::array<LinearCombination, 8> t;
            t[0] = x[j] + x[j + 1];
            t[1] = x[j + 2] + x[j + 3];
            t[2] = x[j + 1] * F(2) + t[1];
            t[3] = x[j + 3] * F(2) + t[0];
            t[4] = t[1] * F(4) + t[3];
            t[5] = t[0] * F(4) + t[2];
            t[6] = t[3] + t[5];
            t[7] = t[2] + t[4];
            x[j] = std::move(t[6]);
            x[j + 1] = std::move(t[5]);
            x[j + 2] = std::move(t[7]);
            x[j + 3] = std::move(t[4]);
        }
    }

    constexpr static void external(std::array<LinearCombination, T>& x) {
        if constexpr (T == 2) {
            auto s = sum(x);
            x[0] += s;
            x[1] += s;
        } else if constexpr (T == 3) {
            auto s = sum(x);
            x[0] += s;
            x[1] += s;
            x[2] += s;
        } else if constexpr (T == 4) {
            m4(x);
        } else if constexpr (T == 8 || T == 12 || T == 16 || T == 20 || T == 24) {
            m4(x);
            std::array<LinearCombination, 4> s;
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

    constexpr static void internal(std::array<LinearCombination, T>& x) {
        auto s = sum(x);
        for (std::size_t i = 0; i < T; ++i) {
            x[i] *= Params::m[i];
            x[i] += s;
        }
    }

    constexpr static void rcb(std::size_t round, std::array<LinearCombination, T>& x) {
        for (std::size_t i = 0; i < T; ++i) {
            x[i] += Params::rcb[round * T + i];
        }
    }

    constexpr static void rcp(std::size_t round, std::array<LinearCombination, T>& x) {
        x[0] += Params::rcp[round];
    }

    constexpr static void rce(std::size_t round, std::array<LinearCombination, T>& x) {
        for (std::size_t i = 0; i < T; ++i) {
            x[i] += Params::rce[round * T + i];
        }
    }

    constexpr static void sboxp(Builder* circuit, LinearCombination& x) {
        if constexpr (Params::a == 3) {
            if constexpr (Builder::degree() >= 3) {
                auto x3 = circuit->auxiliary();
                (*circuit)(x3 == x * x * x);
                x = x3;
            } else {
                auto x2 = circuit->auxiliary();
                auto x3 = circuit->auxiliary();
                (*circuit)(x2 == x * x);
                (*circuit)(x3 == x * x2);
                x = x3;
            }
        } else if constexpr (Params::a == 5) {
            // Lessen constraints if degree >= 3
            auto x2 = circuit->auxiliary();
            auto x4 = circuit->auxiliary();
            auto x5 = circuit->auxiliary();
            (*circuit)(x2 == x * x);
            (*circuit)(x4 == x2 * x2);
            (*circuit)(x5 == x * x4);
            x = x5;
        } else if constexpr (Params::a == 17) {
            // Lessen constraints if 16 >= degree >= 4
            if constexpr (Builder::degree() >= 17) {
                auto x17 = circuit->auxiliary();
                (*circuit)(x17 == x * x * x * x * x * x * x * x * x * x * x * x * x * x * x * x * x);
                x = x17;
            } else if constexpr (Builder::degree() >= 3) {
                auto x3 = circuit->auxiliary();
                auto x9 = circuit->auxiliary();
                auto x15 = circuit->auxiliary();
                auto x17 = circuit->auxiliary();
                (*circuit)(x3 == x * x * x);
                (*circuit)(x9 == x3 * x3 * x3);
                (*circuit)(x15 == x3 * x3 * x9);
                (*circuit)(x17 == x * x * x15);
                x = x17;
            } else {
                auto x2 = circuit->auxiliary();
                auto x4 = circuit->auxiliary();
                auto x8 = circuit->auxiliary();
                auto x16 = circuit->auxiliary();
                auto x17 = circuit->auxiliary();
                (*circuit)(x2 == x * x);
                (*circuit)(x4 == x2 * x2);
                (*circuit)(x8 == x4 * x4);
                (*circuit)(x16 == x8 * x8);
                (*circuit)(x17 == x * x16);
                x = x17;
            }
        } else {
            static_assert(false, "Not implemented");
        }
    }

    constexpr static void sbox(Builder* circuit, std::array<LinearCombination, T>& x) {
        for (std::size_t i = 0; i < T; ++i)
            sboxp(circuit, x[i]);
    }
public:
    constexpr static void permute(Builder* circuit, std::array<LinearCombination, T>& x) {
        auto scope = circuit->scope("Poseidon2::permute");

        Circuit::external(x);

        for (std::size_t round = 0; round < Params::rb; ++round) {
            Circuit::rcb(round, x);
            Circuit::sbox(circuit, x);
            Circuit::external(x);
        }

        for (std::size_t round = 0; round < Params::rp; ++round) {
            Circuit::rcp(round, x);
            Circuit::sboxp(circuit, x[0]);
            Circuit::internal(x);
        }

        for (std::size_t round = 0; round < Params::re; ++round) {
            Circuit::rce(round, x);
            Circuit::sbox(circuit, x);
            Circuit::external(x);
        }
    }
};

template<std::size_t Degree>
struct Assigner {
private:
    constexpr static void sboxp(F& x, std::vector<F>* assigment) {
        if constexpr (Params::a == 3) {
            if constexpr (Degree >= 3) {
                assigment->push_back(
                    x *= x.square()
                );
            } else {
                assigment->push_back(
                    x *= assigment->emplace_back(
                        x.square())
                );
            }
        } else if constexpr (Params::a == 5) {
            assigment->push_back(
                x *= assigment->emplace_back(assigment->emplace_back(
                    x.square()).square())
            );
        } else if constexpr (Params::a == 17) {
            if constexpr (Degree >= 17) {
                assigment->push_back(
                    x *= x.square().square().square().square()
                );
            } else if constexpr (Degree >= 3) {
                F x2{ x.square() };
                F x3{ x * x2 };
                F x6{ x3.square() };
                F x9{ x3 * x6 };
                F x15{ x6 * x9 };
                F x17{ x2 * x15 };
                x = x17;
                assigment->emplace_back(std::move(x3));
                assigment->emplace_back(std::move(x9));
                assigment->emplace_back(std::move(x15));
                assigment->emplace_back(std::move(x17));
            } else {
                assigment->push_back(
                    x *= assigment->emplace_back(assigment->emplace_back(assigment->emplace_back(assigment->emplace_back(
                        x.square()).square()).square()).square())
                );
            }
        } else {
            static_assert(false, "Not implemented");
        }
    }

    constexpr static void sbox(std::array<F, T>& x, std::vector<F>* assigment) {
        for (std::size_t i = 0; i < T; ++i)
            sboxp(x[i], assigment);
    }
public:
    constexpr static void permute(std::array<F, T>& x, std::vector<F>* assigment) {
        external(x);

        for (std::size_t round = 0; round < Params::rb; ++round) {
            rcb(round, x);
            sbox(x, assigment);
            external(x);
        }

        for (std::size_t round = 0; round < Params::rp; ++round) {
            rcp(round, x);
            sboxp(x[0], assigment);
            internal(x);
        }

        for (std::size_t round = 0; round < Params::re; ++round) {
            rce(round, x);
            sbox(x, assigment);
            external(x);
        }
    }
};
};

}

#endif
