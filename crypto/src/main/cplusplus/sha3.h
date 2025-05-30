#ifndef BLACKNET_CRYPTO_SHA3_H
#define BLACKNET_CRYPTO_SHA3_H

// Copyright 2025 Christian Mazakas.
// Distributed under the Boost Software License, Version 1.0.
// https://www.boost.org/LICENSE_1_0.txt
//

// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
// https://github.com/XKCP/XKCP/blob/master/Standalone/CompactFIPS202/C/Keccak-readable-and-compact.c
// https://keccak.team/files/Keccak-reference-3.0.pdf

#include <cassert>
#include <cstdint>
#include <cstring>
#include <array>
#include <bit>

#include "byte.h"
#include "keccak.h"

namespace blacknet
{
namespace crypto
{
namespace wtf
{

template<std::uint8_t PaddingDelim, int C, int D>
struct keccak_base
{
private:

    static constexpr int R = 1600 - C;

    std::byte state_[ 200 ] = {};
    std::size_t m_ = 0;

    bool finalized_ = false;

public:
    template<std::size_t N>
    using digest = std::array<std::byte, N>;

    using result_type = digest<D / 8>;
    static constexpr std::size_t block_size = R / 8;

    void update( void const* pv, std::size_t n )
    {
        std::byte const* p = static_cast<std::byte const*>( pv );
        update( p, n );
    }

    constexpr void update( std::byte const* p, std::size_t n )
    {
        finalized_ = false;

        auto const block_len = R / 8;

        if( m_ > 0 )
        {
            std::size_t k = block_len - m_;

            if( n < k )
            {
                k = n;
            }

            for( std::size_t i = 0; i < k; ++i )
            {
                state_[ m_ + i ] ^= p[ i ];
            }

            p += k;
            n -= k;
            m_ += k;

            if( m_ < block_len ) return;

            assert( m_ == block_len );

            keccak_permute( state_ );
            m_ = 0;
        }

        while( n >= block_len )
        {
            for( int i = 0; i < block_len; ++i )
            {
                state_[ i ] ^= p[ i ];
            }

            keccak_permute( state_ );

            p += block_len;
            n -= block_len;
        }

        assert( n < block_len );

        if( n > 0 )
        {
            for( std::size_t i = 0; i < n; ++i )
            {
                state_[ i ] ^= p[ i ];
            }

            m_ = n;
        }

        assert( m_ == n % block_len );
    }

    constexpr result_type result()
    {
        result_type digest;

        if( !finalized_ )
        {
            state_[ m_ ] ^= std::byte{PaddingDelim};
            state_[ R / 8 - 1 ] ^= std::byte{0x80};

            m_ = 0;
            finalized_ = true;
        }

        keccak_permute( state_ );
        std::memcpy( digest.data(), state_, digest.size() );

        return digest;
    }
};

} // namespace detail

class sha3_224: public wtf::keccak_base<0x06, 2 * 224, 224>
{
public:

    constexpr sha3_224() noexcept = default;
};

class sha3_256: public wtf::keccak_base<0x06, 2 * 256, 256>
{
public:

    constexpr sha3_256() noexcept = default;
};

class sha3_384: public wtf::keccak_base<0x06, 2 * 384, 384>
{
public:

    constexpr sha3_384() noexcept = default;
};

class sha3_512: public wtf::keccak_base<0x06, 2 * 512, 512>
{
public:

    constexpr sha3_512() noexcept = default;
};

class shake_128: public wtf::keccak_base<0x1f, 256, 1600 - 256>
{
public:

    constexpr shake_128() noexcept = default;
};

class shake_256: public wtf::keccak_base<0x1f, 512, 1600 - 512>
{
public:

    constexpr shake_256() noexcept = default;
};

} // namespace hash2
} // namespace boost

#endif // BOOST_HASH2_SHA3_HPP_INCLUDED
