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

#ifndef BLACKNET_CRYPTO_DISCRETEGAUSSIANDISTRIBUTION_H
#define BLACKNET_CRYPTO_DISCRETEGAUSSIANDISTRIBUTION_H

#include <cmath>
#include <type_traits>
#include <boost/random/uniform_int_distribution.hpp>
#include <boost/random/uniform_real_distribution.hpp>

// https://eprint.iacr.org/2007/432
// SampleZ

template<typename T>
struct DiscreteGaussianDistribution {
    static_assert(std::is_signed_v<T>);

    using result_type = T;

    double mu;
    double sigma;
    constexpr static const std::size_t n = 128;

    constexpr DiscreteGaussianDistribution(double mu, double sigma) : mu(mu), sigma(sigma) {}

    template<typename G>
    constexpr result_type operator () (G& g) const {
        // https://eprint.iacr.org/2015/953
        boost::random::uniform_int_distribution<T> uid(min(), max());
        boost::random::uniform_real_distribution urd;
        while (true) {
            result_type x(uid(g));
            double ps = std::exp(- (x - mu) * (x - mu) / (2.0 * sigma * sigma));
            if (urd(g) > ps)
                continue;
            return x;
        }
    }

    constexpr T min() const {
        constexpr double t = std::log2(n);
        return std::floor(mu - sigma * t);
    }
    constexpr T max() const {
        constexpr double t = std::log2(n);
        return std::ceil(mu + sigma * t);
    }
};

#endif
