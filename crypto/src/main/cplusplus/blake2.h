#ifndef BLACKNET_CRYPTO_BLAKE2_H
#define BLACKNET_CRYPTO_BLAKE2_H

// Copyright 2025 Christian Mazakas.
// Copyright 2025 rat4.
// Distributed under the Boost Software License, Version 1.0.
// https://www.boost.org/LICENSE_1_0.txt
//
// https://www.blake2.net/blake2.pdf

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <array>
#include <bit>

#include "byte.h"

namespace blacknet
{
namespace crypto
{
namespace wtf
{

template<class = void>
struct blake2b_constants
{
    constexpr static const std::uint64_t iv[ 8 ] =
    {
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    };
};

template<class = void>
struct blake2s_constants
{
    constexpr static const std::uint32_t iv[ 8 ] =
    {
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    };
};

} // namespace detail

template<std::uint64_t outlen>
class blake2b_base
{
private:

    std::byte b_[ 128 ] = {};
    std::uint64_t h_[ 8 ] = {};
    std::uint64_t t_[ 2 ] = {};
    std::size_t m_ = 0;

    constexpr void init( std::uint64_t keylen = 0 )
    {
        for( int i = 0; i < 8; ++i )
        {
            h_[ i ] = wtf::blake2b_constants<>::iv[ i ];
        }

        h_[ 0 ] ^= 0x01010000 ^ ( keylen << 8 ) ^ outlen;
    }

    constexpr inline static void G( std::uint64_t v[ 16 ], int a, int b, int c, int d, std::uint64_t x, std::uint64_t y )
    {
        v[ a ] = v[ a ] + v[ b ] + x;
        v[ d ] = std::rotr( v[ d ] ^ v[ a ], 32 );
        v[ c ] = v[ c ] + v[ d ];
        v[ b ] = std::rotr( v[ b ] ^ v[ c ], 24 );
        v[ a ] = v[ a ] + v[ b ] + y;
        v[ d ] = std::rotr( v[ d ] ^ v[ a ], 16 );
        v[ c ] = v[ c ] + v[ d ];
        v[ b ] = std::rotr( v[ b ] ^ v[ c ], 63 );
    }

    constexpr void transform( std::byte const block[ 128 ], bool is_final = false )
    {
        auto iv = wtf::blake2b_constants<>::iv;

        std::uint64_t v[ 16 ] = {};
        std::uint64_t m[ 16 ] = {};

        v[ 0 ] = h_[ 0 ];
        v[ 1 ] = h_[ 1 ];
        v[ 2 ] = h_[ 2 ];
        v[ 3 ] = h_[ 3 ];
        v[ 4 ] = h_[ 4 ];
        v[ 5 ] = h_[ 5 ];
        v[ 6 ] = h_[ 6 ];
        v[ 7 ] = h_[ 7 ];

        v[  8 ] = iv[ 0 ];
        v[  9 ] = iv[ 1 ];
        v[ 10 ] = iv[ 2 ];
        v[ 11 ] = iv[ 3 ];
        v[ 12 ] = iv[ 4 ];
        v[ 13 ] = iv[ 5 ];
        v[ 14 ] = iv[ 6 ];
        v[ 15 ] = iv[ 7 ];

        v[ 12 ] ^= t_[ 0 ];
        v[ 13 ] ^= t_[ 1 ];

        if( is_final )
        {
            v[ 14 ] = ~v[ 14 ];
        }

        for( int i = 0; i < 16; ++i )
        {
            m[ i ] = compat::byte::read<std::uint64_t,std::endian::little>( block + 8 * i );
        }

        // round 0
        G( v, 0, 4,  8, 12, m[  0 ], m[  1 ] );
        G( v, 1, 5,  9, 13, m[  2 ], m[  3 ] );
        G( v, 2, 6, 10, 14, m[  4 ], m[  5 ] );
        G( v, 3, 7, 11, 15, m[  6 ], m[  7 ] );
        G( v, 0, 5, 10, 15, m[  8 ], m[  9 ] );
        G( v, 1, 6, 11, 12, m[ 10 ], m[ 11 ] );
        G( v, 2, 7,  8, 13, m[ 12 ], m[ 13 ] );
        G( v, 3, 4,  9, 14, m[ 14 ], m[ 15 ] );

        // round 1
        G( v, 0, 4,  8, 12, m[ 14 ], m[ 10 ] );
        G( v, 1, 5,  9, 13, m[  4 ], m[  8 ] );
        G( v, 2, 6, 10, 14, m[  9 ], m[ 15 ] );
        G( v, 3, 7, 11, 15, m[ 13 ], m[  6 ] );
        G( v, 0, 5, 10, 15, m[  1 ], m[ 12 ] );
        G( v, 1, 6, 11, 12, m[  0 ], m[  2 ] );
        G( v, 2, 7,  8, 13, m[ 11 ], m[  7 ] );
        G( v, 3, 4,  9, 14, m[  5 ], m[  3 ] );

        // round 2
        G( v, 0, 4,  8, 12, m[ 11 ], m[  8 ] );
        G( v, 1, 5,  9, 13, m[ 12 ], m[  0 ] );
        G( v, 2, 6, 10, 14, m[  5 ], m[  2 ] );
        G( v, 3, 7, 11, 15, m[ 15 ], m[ 13 ] );
        G( v, 0, 5, 10, 15, m[ 10 ], m[ 14 ] );
        G( v, 1, 6, 11, 12, m[  3 ], m[  6 ] );
        G( v, 2, 7,  8, 13, m[  7 ], m[  1 ] );
        G( v, 3, 4,  9, 14, m[  9 ], m[  4 ] );

        // round 3
        G( v, 0, 4,  8, 12, m[  7 ], m[  9 ] );
        G( v, 1, 5,  9, 13, m[  3 ], m[  1 ] );
        G( v, 2, 6, 10, 14, m[ 13 ], m[ 12 ] );
        G( v, 3, 7, 11, 15, m[ 11 ], m[ 14 ] );
        G( v, 0, 5, 10, 15, m[  2 ], m[  6 ] );
        G( v, 1, 6, 11, 12, m[  5 ], m[ 10 ] );
        G( v, 2, 7,  8, 13, m[  4 ], m[  0 ] );
        G( v, 3, 4,  9, 14, m[ 15 ], m[  8 ] );

        // round 4
        G( v, 0, 4,  8, 12, m[  9 ], m[  0 ] );
        G( v, 1, 5,  9, 13, m[  5 ], m[  7 ] );
        G( v, 2, 6, 10, 14, m[  2 ], m[  4 ] );
        G( v, 3, 7, 11, 15, m[ 10 ], m[ 15 ] );
        G( v, 0, 5, 10, 15, m[ 14 ], m[  1 ] );
        G( v, 1, 6, 11, 12, m[ 11 ], m[ 12 ] );
        G( v, 2, 7,  8, 13, m[  6 ], m[  8 ] );
        G( v, 3, 4,  9, 14, m[  3 ], m[ 13 ] );

        // round 5
        G( v, 0, 4,  8, 12,  m[  2 ], m[ 12 ] );
        G( v, 1, 5,  9, 13,  m[  6 ], m[ 10 ] );
        G( v, 2, 6, 10, 14,  m[  0 ], m[ 11 ] );
        G( v, 3, 7, 11, 15,  m[  8 ], m[  3 ] );
        G( v, 0, 5, 10, 15,  m[  4 ], m[ 13 ] );
        G( v, 1, 6, 11, 12,  m[  7 ], m[  5 ] );
        G( v, 2, 7,  8, 13,  m[ 15 ], m[ 14 ] );
        G( v, 3, 4,  9, 14,  m[  1 ], m[  9 ] );

        // round 6
        G( v, 0, 4,  8, 12, m[ 12 ], m[  5 ] );
        G( v, 1, 5,  9, 13, m[  1 ], m[ 15 ] );
        G( v, 2, 6, 10, 14, m[ 14 ], m[ 13 ] );
        G( v, 3, 7, 11, 15, m[  4 ], m[ 10 ] );
        G( v, 0, 5, 10, 15, m[  0 ], m[  7 ] );
        G( v, 1, 6, 11, 12, m[  6 ], m[  3 ] );
        G( v, 2, 7,  8, 13, m[  9 ], m[  2 ] );
        G( v, 3, 4,  9, 14, m[  8 ], m[ 11 ] );

        // round 7
        G( v, 0, 4,  8, 12, m[ 13 ], m[ 11 ] );
        G( v, 1, 5,  9, 13, m[  7 ], m[ 14 ] );
        G( v, 2, 6, 10, 14, m[ 12 ], m[  1 ] );
        G( v, 3, 7, 11, 15, m[  3 ], m[  9 ] );
        G( v, 0, 5, 10, 15, m[  5 ], m[  0 ] );
        G( v, 1, 6, 11, 12, m[ 15 ], m[  4 ] );
        G( v, 2, 7,  8, 13, m[  8 ], m[  6 ] );
        G( v, 3, 4,  9, 14, m[  2 ], m[ 10 ] );

        // round 8
        G( v, 0, 4,  8, 12, m[  6 ], m[ 15 ] );
        G( v, 1, 5,  9, 13, m[ 14 ], m[  9 ] );
        G( v, 2, 6, 10, 14, m[ 11 ], m[  3 ] );
        G( v, 3, 7, 11, 15, m[  0 ], m[  8 ] );
        G( v, 0, 5, 10, 15, m[ 12 ], m[  2 ] );
        G( v, 1, 6, 11, 12, m[ 13 ], m[  7 ] );
        G( v, 2, 7,  8, 13, m[  1 ], m[  4 ] );
        G( v, 3, 4,  9, 14, m[ 10 ], m[  5 ] );

        // round 9
        G( v, 0, 4,  8, 12, m[ 10 ], m[  2 ] );
        G( v, 1, 5,  9, 13, m[  8 ], m[  4 ] );
        G( v, 2, 6, 10, 14, m[  7 ], m[  6 ] );
        G( v, 3, 7, 11, 15, m[  1 ], m[  5 ] );
        G( v, 0, 5, 10, 15, m[ 15 ], m[ 11 ] );
        G( v, 1, 6, 11, 12, m[  9 ], m[ 14 ] );
        G( v, 2, 7,  8, 13, m[  3 ], m[ 12 ] );
        G( v, 3, 4,  9, 14, m[ 13 ], m[  0 ] );

        // round 10
        G( v, 0, 4,  8, 12, m[  0 ], m[  1 ] );
        G( v, 1, 5,  9, 13, m[  2 ], m[  3 ] );
        G( v, 2, 6, 10, 14, m[  4 ], m[  5 ] );
        G( v, 3, 7, 11, 15, m[  6 ], m[  7 ] );
        G( v, 0, 5, 10, 15, m[  8 ], m[  9 ] );
        G( v, 1, 6, 11, 12, m[ 10 ], m[ 11 ] );
        G( v, 2, 7,  8, 13, m[ 12 ], m[ 13 ] );
        G( v, 3, 4,  9, 14, m[ 14 ], m[ 15 ] );

        // round 11
        G( v, 0, 4,  8, 12, m[ 14 ], m[ 10 ] );
        G( v, 1, 5,  9, 13, m[  4 ], m[  8 ] );
        G( v, 2, 6, 10, 14, m[  9 ], m[ 15 ] );
        G( v, 3, 7, 11, 15, m[ 13 ], m[  6 ] );
        G( v, 0, 5, 10, 15, m[  1 ], m[ 12 ] );
        G( v, 1, 6, 11, 12, m[  0 ], m[  2 ] );
        G( v, 2, 7,  8, 13, m[ 11 ], m[  7 ] );
        G( v, 3, 4,  9, 14, m[  5 ], m[  3 ] );

        h_[ 0 ] ^= v[ 0 ] ^ v[ 0 + 8 ];
        h_[ 1 ] ^= v[ 1 ] ^ v[ 1 + 8 ];
        h_[ 2 ] ^= v[ 2 ] ^ v[ 2 + 8 ];
        h_[ 3 ] ^= v[ 3 ] ^ v[ 3 + 8 ];
        h_[ 4 ] ^= v[ 4 ] ^ v[ 4 + 8 ];
        h_[ 5 ] ^= v[ 5 ] ^ v[ 5 + 8 ];
        h_[ 6 ] ^= v[ 6 ] ^ v[ 6 + 8 ];
        h_[ 7 ] ^= v[ 7 ] ^ v[ 7 + 8 ];
    }

    constexpr inline void incr_len( std::size_t n )
    {
        t_[ 0 ] += n;
        t_[ 1 ] += ( t_[ 0 ] < n ); // overflowed
    }

public:
    template<std::size_t N>
    using digest = std::array<std::byte, N>;

    using result_type = digest<outlen>;

    static constexpr std::size_t block_size = 128;

    constexpr blake2b_base() noexcept
    {
        init();
    }

    void update( void const* pv, std::size_t n )
    {
        std::byte const* p = reinterpret_cast<std::byte const*>( pv );
        update( p, n );
    }

    constexpr void update( std::byte const* p, std::size_t n )
    {
        if( n > 0 )
        {
            std::size_t k = block_size - m_;
            if( n > k )
            {
                std::memcpy( b_ + m_, p, k );
                incr_len( block_size );
                transform( b_ );
                std::memset( b_ , 0, block_size );
                p += k;
                n -= k;
                m_ = 0;

                while( n > block_size )
                {
                    incr_len( block_size );
                    transform( p );
                    p += block_size;
                    n -= block_size;
                }
            }

            std::memcpy( b_ + m_, p, n );
            m_ += n;
        }
    }

    constexpr result_type result()
    {
        result_type digest;

        incr_len( m_ );
        for( auto i = m_; i < block_size; ++i )
        {
            b_[ i ] = std::byte{0};
        }

        transform( b_, true );
        std::memset( b_ , 0,  block_size );
        m_ = 0;

        for( int i = 0; i < 8; ++i )
        {
            compat::byte::write<std::uint64_t,std::endian::little>( digest.data() + i * 8, h_[ i ] );
        }
        return digest;
    }
};

class blake2b_512 : public blake2b_base<64>
{
};

class blake2b_256 : public blake2b_base<32>
{
};

class blake2s_256
{
private:

    std::byte b_[ 64 ] = {};
    std::uint32_t h_[ 8 ] = {};
    std::uint32_t t_[ 2 ] = {};
    std::size_t m_ = 0;

    constexpr void init( std::uint32_t keylen = 0 )
    {
        for( int i = 0; i < 8; ++i )
        {
            h_[ i ] = wtf::blake2s_constants<>::iv[ i ];
        }

        std::uint32_t const outlen = 32;
        h_[ 0 ] ^= 0x01010000 ^ ( keylen << 8 ) ^ outlen;
    }

    constexpr inline static void G( std::uint32_t v[ 16 ], int a, int b, int c, int d, std::uint32_t x, std::uint32_t y )
    {
        v[ a ] = v[ a ] + v[ b ] + x;
        v[ d ] = std::rotr( v[ d ] ^ v[ a ], 16 );
        v[ c ] = v[ c ] + v[ d ];
        v[ b ] = std::rotr( v[ b ] ^ v[ c ], 12 );
        v[ a ] = v[ a ] + v[ b ] + y;
        v[ d ] = std::rotr( v[ d ] ^ v[ a ], 8 );
        v[ c ] = v[ c ] + v[ d ];
        v[ b ] = std::rotr( v[ b ] ^ v[ c ], 7 );
    }

    constexpr void transform( std::byte const block[ 64 ], bool is_final = false )
    {
        auto iv = wtf::blake2s_constants<>::iv;

        std::uint32_t v[ 16 ] = {};
        std::uint32_t m[ 16 ] = {};

        v[ 0 ] = h_[ 0 ];
        v[ 1 ] = h_[ 1 ];
        v[ 2 ] = h_[ 2 ];
        v[ 3 ] = h_[ 3 ];
        v[ 4 ] = h_[ 4 ];
        v[ 5 ] = h_[ 5 ];
        v[ 6 ] = h_[ 6 ];
        v[ 7 ] = h_[ 7 ];

        v[  8 ] = iv[ 0 ];
        v[  9 ] = iv[ 1 ];
        v[ 10 ] = iv[ 2 ];
        v[ 11 ] = iv[ 3 ];
        v[ 12 ] = iv[ 4 ];
        v[ 13 ] = iv[ 5 ];
        v[ 14 ] = iv[ 6 ];
        v[ 15 ] = iv[ 7 ];

        v[ 12 ] ^= t_[ 0 ];
        v[ 13 ] ^= t_[ 1 ];

        if( is_final )
        {
            v[ 14 ] = ~v[ 14 ];
        }

        for( int i = 0; i < 16; ++i )
        {
            m[ i ] = compat::byte::read<std::uint32_t,std::endian::little>( block + 4 * i );
        }

        // round 0
        G( v, 0, 4,  8, 12, m[  0 ], m[  1 ] );
        G( v, 1, 5,  9, 13, m[  2 ], m[  3 ] );
        G( v, 2, 6, 10, 14, m[  4 ], m[  5 ] );
        G( v, 3, 7, 11, 15, m[  6 ], m[  7 ] );
        G( v, 0, 5, 10, 15, m[  8 ], m[  9 ] );
        G( v, 1, 6, 11, 12, m[ 10 ], m[ 11 ] );
        G( v, 2, 7,  8, 13, m[ 12 ], m[ 13 ] );
        G( v, 3, 4,  9, 14, m[ 14 ], m[ 15 ] );

        // round 1
        G( v, 0, 4,  8, 12, m[ 14 ], m[ 10 ] );
        G( v, 1, 5,  9, 13, m[  4 ], m[  8 ] );
        G( v, 2, 6, 10, 14, m[  9 ], m[ 15 ] );
        G( v, 3, 7, 11, 15, m[ 13 ], m[  6 ] );
        G( v, 0, 5, 10, 15, m[  1 ], m[ 12 ] );
        G( v, 1, 6, 11, 12, m[  0 ], m[  2 ] );
        G( v, 2, 7,  8, 13, m[ 11 ], m[  7 ] );
        G( v, 3, 4,  9, 14, m[  5 ], m[  3 ] );

        // round 2
        G( v, 0, 4,  8, 12, m[ 11 ], m[  8 ] );
        G( v, 1, 5,  9, 13, m[ 12 ], m[  0 ] );
        G( v, 2, 6, 10, 14, m[  5 ], m[  2 ] );
        G( v, 3, 7, 11, 15, m[ 15 ], m[ 13 ] );
        G( v, 0, 5, 10, 15, m[ 10 ], m[ 14 ] );
        G( v, 1, 6, 11, 12, m[  3 ], m[  6 ] );
        G( v, 2, 7,  8, 13, m[  7 ], m[  1 ] );
        G( v, 3, 4,  9, 14, m[  9 ], m[  4 ] );

        // round 3
        G( v, 0, 4,  8, 12, m[  7 ], m[  9 ] );
        G( v, 1, 5,  9, 13, m[  3 ], m[  1 ] );
        G( v, 2, 6, 10, 14, m[ 13 ], m[ 12 ] );
        G( v, 3, 7, 11, 15, m[ 11 ], m[ 14 ] );
        G( v, 0, 5, 10, 15, m[  2 ], m[  6 ] );
        G( v, 1, 6, 11, 12, m[  5 ], m[ 10 ] );
        G( v, 2, 7,  8, 13, m[  4 ], m[  0 ] );
        G( v, 3, 4,  9, 14, m[ 15 ], m[  8 ] );

        // round 4
        G( v, 0, 4,  8, 12, m[  9 ], m[  0 ] );
        G( v, 1, 5,  9, 13, m[  5 ], m[  7 ] );
        G( v, 2, 6, 10, 14, m[  2 ], m[  4 ] );
        G( v, 3, 7, 11, 15, m[ 10 ], m[ 15 ] );
        G( v, 0, 5, 10, 15, m[ 14 ], m[  1 ] );
        G( v, 1, 6, 11, 12, m[ 11 ], m[ 12 ] );
        G( v, 2, 7,  8, 13, m[  6 ], m[  8 ] );
        G( v, 3, 4,  9, 14, m[  3 ], m[ 13 ] );

        // round 5
        G( v, 0, 4,  8, 12, m[  2 ], m[ 12 ] );
        G( v, 1, 5,  9, 13, m[  6 ], m[ 10 ] );
        G( v, 2, 6, 10, 14, m[  0 ], m[ 11 ] );
        G( v, 3, 7, 11, 15, m[  8 ], m[  3 ] );
        G( v, 0, 5, 10, 15, m[  4 ], m[ 13 ] );
        G( v, 1, 6, 11, 12, m[  7 ], m[  5 ] );
        G( v, 2, 7,  8, 13, m[ 15 ], m[ 14 ] );
        G( v, 3, 4,  9, 14, m[  1 ], m[  9 ] );

        // round 6
        G( v, 0, 4,  8, 12, m[ 12 ], m[  5 ] );
        G( v, 1, 5,  9, 13, m[  1 ], m[ 15 ] );
        G( v, 2, 6, 10, 14, m[ 14 ], m[ 13 ] );
        G( v, 3, 7, 11, 15, m[  4 ], m[ 10 ] );
        G( v, 0, 5, 10, 15, m[  0 ], m[  7 ] );
        G( v, 1, 6, 11, 12, m[  6 ], m[  3 ] );
        G( v, 2, 7,  8, 13, m[  9 ], m[  2 ] );
        G( v, 3, 4,  9, 14, m[  8 ], m[ 11 ] );

        // round 7
        G( v, 0, 4,  8, 12, m[ 13 ], m[ 11 ] );
        G( v, 1, 5,  9, 13, m[  7 ], m[ 14 ] );
        G( v, 2, 6, 10, 14, m[ 12 ], m[  1 ] );
        G( v, 3, 7, 11, 15, m[  3 ], m[  9 ] );
        G( v, 0, 5, 10, 15, m[  5 ], m[  0 ] );
        G( v, 1, 6, 11, 12, m[ 15 ], m[  4 ] );
        G( v, 2, 7,  8, 13, m[  8 ], m[  6 ] );
        G( v, 3, 4,  9, 14, m[  2 ], m[ 10 ] );

        // round 8
        G( v, 0, 4,  8, 12, m[  6 ], m[ 15 ] );
        G( v, 1, 5,  9, 13, m[ 14 ], m[  9 ] );
        G( v, 2, 6, 10, 14, m[ 11 ], m[  3 ] );
        G( v, 3, 7, 11, 15, m[  0 ], m[  8 ] );
        G( v, 0, 5, 10, 15, m[ 12 ], m[  2 ] );
        G( v, 1, 6, 11, 12, m[ 13 ], m[  7 ] );
        G( v, 2, 7,  8, 13, m[  1 ], m[  4 ] );
        G( v, 3, 4,  9, 14, m[ 10 ], m[  5 ] );

        // round 9
        G( v, 0, 4,  8, 12, m[ 10 ], m[  2 ] );
        G( v, 1, 5,  9, 13, m[  8 ], m[  4 ] );
        G( v, 2, 6, 10, 14, m[  7 ], m[  6 ] );
        G( v, 3, 7, 11, 15, m[  1 ], m[  5 ] );
        G( v, 0, 5, 10, 15, m[ 15 ], m[ 11 ] );
        G( v, 1, 6, 11, 12, m[  9 ], m[ 14 ] );
        G( v, 2, 7,  8, 13, m[  3 ], m[ 12 ] );
        G( v, 3, 4,  9, 14, m[ 13 ], m[  0 ] );

        h_[ 0 ] ^= v[ 0 ] ^ v[ 0 + 8 ];
        h_[ 1 ] ^= v[ 1 ] ^ v[ 1 + 8 ];
        h_[ 2 ] ^= v[ 2 ] ^ v[ 2 + 8 ];
        h_[ 3 ] ^= v[ 3 ] ^ v[ 3 + 8 ];
        h_[ 4 ] ^= v[ 4 ] ^ v[ 4 + 8 ];
        h_[ 5 ] ^= v[ 5 ] ^ v[ 5 + 8 ];
        h_[ 6 ] ^= v[ 6 ] ^ v[ 6 + 8 ];
        h_[ 7 ] ^= v[ 7 ] ^ v[ 7 + 8 ];
    }

    constexpr void incr_len( std::size_t n )
    {
        auto m = static_cast<std::uint32_t>( n );
        t_[ 0 ] += m;
        t_[ 1 ] += ( t_[ 0 ] < m ); // overflowed
    }

public:
    template<std::size_t N>
    using digest = std::array<std::byte, N>;

    using result_type = digest<32>;

    static constexpr std::size_t block_size = 64;

    constexpr blake2s_256() noexcept
    {
        init();
    }

    void update( void const* pv, std::size_t n )
    {
        std::byte const* p = reinterpret_cast<std::byte const*>( pv );
        update( p, n );
    }

    constexpr void update( std::byte const* p, std::size_t n )
    {
        if( n > 0 )
        {
            std::size_t k = block_size - m_;
            if( n > k )
            {
                std::memcpy( b_ + m_, p, k );
                incr_len( block_size );
                transform( b_ );
                std::memset( b_ , 0,  block_size );
                p += k;
                n -= k;
                m_ = 0;

                while( n > block_size )
                {
                    incr_len( block_size );
                    transform( p );
                    p += block_size;
                    n -= block_size;
                }
            }

            std::memcpy( b_ + m_, p, n );
            m_ += n;
        }
    }

    constexpr result_type result()
    {
        result_type digest;

        incr_len( m_ );
        for( auto i = m_; i < block_size; ++i )
        {
            b_[ i ] = std::byte{0};
        }

        transform( b_, true );
        std::memset( b_ , 0,  block_size );
        m_ = 0;
        for( int i = 0; i < 8; ++i )
        {
            compat::byte::write<std::uint32_t,std::endian::little>( digest.data() + i * 4, h_[ i ] );
        }
        return digest;
    }
};

} // namespace hash2
} // namespace boost

#endif // BOOST_HASH2_BLAKE2_HPP_INCLUDED
