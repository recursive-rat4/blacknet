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

#ifndef BLACKNET_CRYPTO_UTIL_H
#define BLACKNET_CRYPTO_UTIL_H

#include <array>
#include <iomanip>
#include <utility>
#include <vector>
#include <boost/io/ios_state.hpp>
#include <boost/io/ostream_joiner.hpp>

namespace util {
    template<typename T>
    struct Add {
        constexpr static void call(T& l, T&& r) {
            l += std::move(r);
        }

        constexpr static void call(T& l, const T& r) {
            l += r;
        }

        constexpr static void call(std::vector<T>& l, std::vector<T>&& r) {
            for (std::size_t i = 0; i < r.size(); ++i)
                l[i] += std::move(r[i]);
        }

        constexpr static void call(std::vector<T>& l, const std::vector<T>& r) {
            for (std::size_t i = 0; i < r.size(); ++i)
                l[i] += r[i];
        }
    };

    template<typename T>
    struct Assign {
        constexpr static void call(T& l, T&& r) {
            l = std::move(r);
        }

        constexpr static void call(T& l, const T& r) {
            l = r;
        }

        constexpr static void call(std::vector<T>& l, std::vector<T>&& r) {
            l = std::move(r);
        }

        constexpr static void call(std::vector<T>& l, const std::vector<T>& r) {
            l = r;
        }
    };

    template<typename T>
    struct Sub {
        constexpr static void call(T& l, T&& r) {
            l -= std::move(r);
        }

        constexpr static void call(T& l, const T& r) {
            l -= r;
        }

        constexpr static void call(std::vector<T>& l, std::vector<T>&& r) {
            for (std::size_t i = 0; i < r.size(); ++i)
                l[i] -= std::move(r[i]);
        }

        constexpr static void call(std::vector<T>& l, const std::vector<T>& r) {
            for (std::size_t i = 0; i < r.size(); ++i)
                l[i] -= r[i];
        }
    };

    template<typename T>
    struct Mul {
        constexpr static void call(T& l, T&& r) {
            l *= std::move(r);
        }

        constexpr static void call(T& l, const T& r) {
            l *= r;
        }

        constexpr static void call(std::vector<T>& l, const T& r) {
            for (std::size_t i = 0; i < l.size(); ++i)
                l[i] *= r;
        }

        constexpr static void call(std::vector<T>& l, std::vector<T>&& r) {
            for (std::size_t i = 0; i < r.size(); ++i)
                l[i] *= std::move(r[i]);
        }

        constexpr static void call(std::vector<T>& l, const std::vector<T>& r) {
            for (std::size_t i = 0; i < r.size(); ++i)
                l[i] *= r[i];
        }
    };

    template<typename T>
    struct Sum {
        constexpr static T call(std::vector<T>&& seq) {
            T sum(seq[0]);
            for (std::size_t i = 1; i < seq.size(); ++i)
                sum += std::move(seq[i]);
            return sum;
        }

        constexpr static T call(const std::vector<T>& seq) {
            T sum(seq[0]);
            for (std::size_t i = 1; i < seq.size(); ++i)
                sum += seq[i];
            return sum;
        }
    };
}

namespace std {
    template<typename T, std::size_t N>
    std::ostream& operator << (std::ostream& out, const std::array<T, N>& val)
    {
        out << '[';
        std::copy(val.cbegin(), val.cend(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }

    template<typename T, typename A>
    std::ostream& operator << (std::ostream& out, const std::vector<T, A>& val)
    {
        out << '[';
        std::copy(val.cbegin(), val.cend(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }

    template<typename A, typename B>
    std::ostream& operator << (std::ostream& out, const std::pair<A, B>& val)
    {
        return out << '(' << std::get<0>(val) << ", " << std::get<1>(val) << ')';
    }

    std::ostream& operator << (std::ostream& out, const std::byte val)
    {
        boost::io::ios_all_saver ias(out);
        return out << "0x"
            << std::hex << std::setfill('0') << std::setw(sizeof(std::byte) * 2)
            << static_cast<unsigned>(val);
    }
}

#endif
