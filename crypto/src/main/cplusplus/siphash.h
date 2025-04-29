#ifndef BLACKNET_CRYPTO_SIPHASH_H
#define BLACKNET_CRYPTO_SIPHASH_H

// Copyright 2017, 2018 Peter Dimov.
// Copyright 2025 rat4.
// Distributed under the Boost Software License, Version 1.0.
// https://www.boost.org/LICENSE_1_0.txt
//
// SipHash, https://cr.yp.to/siphash/siphash-20120918.pdf

#include <cassert>
#include <cstdint>
#include <cstring>
#include <cstddef>
#include <bit>
#include <span>

#include "byte.h"

namespace blacknet
{
namespace crypto
{

class siphash_64
{
private:

    std::uint64_t v0 = 0x736f6d6570736575ULL;
    std::uint64_t v1 = 0x646f72616e646f6dULL;
    std::uint64_t v2 = 0x6c7967656e657261ULL;
    std::uint64_t v3 = 0x7465646279746573ULL;

    std::byte buffer_[ 8 ] = {};
    std::size_t m_ = 0; // == n_ % 8

    std::uint64_t n_ = 0;

private:

    constexpr void sipround()
    {
        v0 += v1;
        v1 = std::rotl(v1, 13);
        v1 ^= v0;
        v0 = std::rotl(v0, 32);
        v2 += v3;
        v3 = std::rotl(v3, 16);
        v3 ^= v2;
        v0 += v3;
        v3 = std::rotl(v3, 21);
        v3 ^= v0;
        v2 += v1;
        v1 = std::rotl(v1, 17);
        v1 ^= v2;
        v2 = std::rotl(v2, 32);
    }

    constexpr void update_( std::byte const * p )
    {
        std::uint64_t m = compat::byte::read<std::uint64_t,std::endian::little>( p );

        v3 ^= m;

        sipround();
        sipround();

        v0 ^= m;
    }

public:

    using result_type = std::uint64_t;

    constexpr siphash_64( const std::span<const std::byte, 16>& key )
    {
        std::uint64_t k0 = compat::byte::read<std::uint64_t,std::endian::little>( key.data() + 0 );
        std::uint64_t k1 = compat::byte::read<std::uint64_t,std::endian::little>( key.data() + 8 );

        v0 ^= k0;
        v1 ^= k1;
        v2 ^= k0;
        v3 ^= k1;
    }

    constexpr void update( std::byte const* p, std::size_t n )
    {
        assert( m_ == n_ % 8 );

        if( n == 0 ) return;

        n_ += n;

        if( m_ > 0 )
        {
            std::size_t k = 8 - m_;

            if( n < k )
            {
                k = n;
            }

            std::memcpy( buffer_ + m_, p, k );

            p += k;
            n -= k;
            m_ += k;

            if( m_ < 8 ) return;

            assert( m_ == 8 );

            update_( buffer_ );
            m_ = 0;

            // clear buffered plaintext
            std::memset( buffer_, 0, 8 );
        }

        assert( m_ == 0 );

        while( n >= 8 )
        {
            update_( p );

            p += 8;
            n -= 8;
        }

        assert( n < 8 );

        if( n > 0 )
        {
            std::memcpy( buffer_, p, n );
            m_ = n;
        }

        assert( m_ == n_ % 8 );
    }

    void update( void const* pv, std::size_t n )
    {
        std::byte const* p = static_cast<std::byte const*>( pv );
        update( p, n );
    }

    void update( std::byte v )
    {
        update( &v, 1 );
    }

    constexpr std::uint64_t result()
    {
        assert( m_ == n_ % 8 );

        std::memset( buffer_ + m_, 0, 8 - m_ );

        buffer_[ 7 ] = static_cast<std::byte>( n_ & 0xFF );

        update_( buffer_ );

        v2 ^= 0xFF;

        sipround();
        sipround();
        sipround();
        sipround();

        n_ += 8 - m_;
        m_ = 0;

        // clear buffered plaintext
        std::memset( buffer_, 0, 8 );

        return v0 ^ v1 ^ v2 ^ v3;
    }
};

class siphash_32
{
private:

    std::uint32_t v0 = 0;
    std::uint32_t v1 = 0;
    std::uint32_t v2 = 0x6c796765;
    std::uint32_t v3 = 0x74656462;

    std::byte buffer_[ 4 ] = {};
    std::uint32_t m_ = 0; // == n_ % 4

    std::uint32_t n_ = 0;

private:

    constexpr void sipround()
    {
        v0 += v1;
        v1 = std::rotl(v1, 5);
        v1 ^= v0;
        v0 = std::rotl(v0, 16);
        v2 += v3;
        v3 = std::rotl(v3, 8);
        v3 ^= v2;
        v0 += v3;
        v3 = std::rotl(v3, 7);
        v3 ^= v0;
        v2 += v1;
        v1 = std::rotl(v1, 13);
        v1 ^= v2;
        v2 = std::rotl(v2, 16);
    }

    constexpr void update_( std::byte const * p )
    {
        std::uint32_t m = compat::byte::read<std::uint32_t,std::endian::little>( p );

        v3 ^= m;

        sipround();
        sipround();

        v0 ^= m;
    }

public:

    using result_type = std::uint32_t;

    constexpr siphash_32( const std::span<const std::byte, 8>& key )
    {
        std::uint32_t k0 = compat::byte::read<std::uint32_t,std::endian::little>( key.data() + 0 );
        std::uint32_t k1 = compat::byte::read<std::uint32_t,std::endian::little>( key.data() + 4 );

        v0 ^= k0;
        v1 ^= k1;
        v2 ^= k0;
        v3 ^= k1;
    }

    constexpr void update( std::byte const* p, std::size_t n )
    {
        assert( m_ == n_ % 4 );

        if( n == 0 ) return;

        n_ += static_cast<std::uint32_t>( n );

        if( m_ > 0 )
        {
            std::uint32_t k = 4 - m_;

            if( n < k )
            {
                k = static_cast<std::uint32_t>( n );
            }

            std::memcpy( buffer_ + m_, p, k );

            p += k;
            n -= k;
            m_ += k;

            if( m_ < 4 ) return;

            assert( m_ == 4 );

            update_( buffer_ );
            m_ = 0;

            // clear buffered plaintext
            std::memset( buffer_, 0, 4 );
        }

        assert( m_ == 0 );

        while( n >= 4 )
        {
            update_( p );

            p += 4;
            n -= 4;
        }

        assert( n < 4 );

        if( n > 0 )
        {
            std::memcpy( buffer_, p, n );
            m_ = static_cast<std::uint32_t>( n );
        }

        assert( m_ == n_ % 4 );
    }

    void update( void const* pv, std::size_t n )
    {
        std::byte const* p = static_cast<std::byte const*>( pv );
        update( p, n );
    }

    void update( std::byte v )
    {
        update( &v, 1 );
    }

    constexpr std::uint32_t result()
    {
        assert( m_ == n_ % 4 );

        std::memset( buffer_ + m_, 0, 4 - m_ );

        buffer_[ 3 ] = static_cast<std::byte>( n_ & 0xFF );

        update_( buffer_ );

        v2 ^= 0xFF;

        sipround();
        sipround();
        sipround();
        sipround();

        n_ += 4 - m_;
        m_ = 0;

        // clear buffered plaintext
        std::memset( buffer_, 0, 4 );

        return v1 ^ v3;
    }
};

} // namespace hash2
} // namespace boost

#endif // #ifndef BOOST_HASH2_SIPHASH_HPP_INCLUDED
