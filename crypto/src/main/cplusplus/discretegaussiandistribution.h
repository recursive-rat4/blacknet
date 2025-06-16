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

#ifndef BLACKNET_CRYPTO_DISCRETEGAUSSIANDISTRIBUTION_H
#define BLACKNET_CRYPTO_DISCRETEGAUSSIANDISTRIBUTION_H

#include <bit>
#include <cmath>
#include <random>
#include <type_traits>

namespace blacknet::crypto {

// https://eprint.iacr.org/2007/432
// SampleZ

template<typename T>
struct DiscreteGaussianDistribution {
    static_assert(std::is_signed_v<T>);

    using result_type = T;

    double mu;
    double sigma;
    constexpr static const std::size_t n = 128;
    static_assert(std::has_single_bit(n), "Not implemented");

    constexpr DiscreteGaussianDistribution(double mu, double sigma) : mu(mu), sigma(sigma) {}

    constexpr void reset() noexcept {}

    template<std::uniform_random_bit_generator RNG>
    constexpr result_type operator () (RNG& rng) const {
        // https://eprint.iacr.org/2015/953
        std::uniform_int_distribution<T> uid(min(), max());
        std::uniform_real_distribution<double> urd;
        while (true) {
            result_type x(uid(rng));
            double ps = std::exp(- (x - mu) * (x - mu) / (2.0 * sigma * sigma));
            if (urd(rng) > ps)
                continue;
            return x;
        }
    }

    constexpr result_type min() const {
        constexpr double t = std::countr_zero(n);
        return std::floor(mu - sigma * t);
    }
    constexpr result_type max() const {
        constexpr double t = std::countr_zero(n);
        return std::ceil(mu + sigma * t);
    }
};

}

#endif
