#!/usr/bin/env sage
#
# Copyright (c) 2024 Pavel Vasin
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
#

from typing import NamedTuple
from sympy.ntheory.residue_ntheory import nthroot_mod

class RingSpec(NamedTuple):
    file_name: str
    comment: str
    type_name: str
    modulus: int
    cyclotomic_degree: int
    reduce: str

class RingParams(NamedTuple):
    bits: int
    word_bits: int
    square_montgomery_modulus: int
    montgomery_modulus: int
    zetas: list[int]

def compute_bits(number):
    return ceil(log(number, 2.0))

def compute_word_bits(number):
    bits = compute_bits(ring.modulus)
    return 2**(ceil(log(float(bits), 2.0)))

def compute_centered_representation(number, modulus):
    if number > modulus / 2:
        return number - modulus
    if number < - modulus / 2:
        return number + modulus
    return number

def compute_square_montgomery_modulus(prime, word_bits):
    word = 2**word_bits
    return word**2 % prime

def compute_montgomery_modulus(prime, word_bits):
    word = 2**word_bits
    return pow(prime, -1, word)

def compute_montgomery_reduce(number, modulus, montgomery_modulus, word_bits):
    t = number * montgomery_modulus % 2**word_bits
    return (number - t * modulus) >> word_bits

def compute_montgomery_form(number, modulus, montgomery_modulus, square_montgomery_modulus, word_bits):
    return compute_montgomery_reduce(number * square_montgomery_modulus, modulus, montgomery_modulus, word_bits)

def compute_primitive_root_of_unity(prime, degree):
    roots = nthroot_mod(1, degree, prime, True)
    for root in roots:
        is_primitive = True
        for n in range(1, degree):
            if pow(root, n, prime) == 1:
                is_primitive = False
                break
        if is_primitive:
            return root
    raise ValueError(f"Found {len(roots)} roots, but none are primitive")

def compute_bitreversal(number, bits):
    reversed = 0
    for i in range(0, bits):
        reversed <<= 1
        reversed |= number & 1
        number >>= 1
    return reversed

def write_ring_cplusplus(spec, params):
    with open(spec.file_name, 'w', encoding="utf-8") as file:
        file.write("// Auto-generated with rings.sage\n")
        file.write('\n')
        file.write(f"#ifndef BLACKNET_CRYPTO_{spec.file_name.upper().replace('.', '_')}\n")
        file.write(f"#define BLACKNET_CRYPTO_{spec.file_name.upper().replace('.', '_')}\n")
        file.write('\n')
        file.write('#include <array>\n')
        file.write('\n')
        file.write('#include "integerring.h"\n')
        file.write('\n')
        file.write(f"struct {spec.type_name}Params" + " {\n")
        file.write(f"    using I = int{params.word_bits}_t;\n")
        dword_bits = params.word_bits * 2
        if dword_bits <= 64:
            file.write(f"    using L = int{dword_bits}_t;\n")
        else:
            file.write(f"    using L = __int{dword_bits}_t;\n")
        file.write(f"    using UI = uint{params.word_bits}_t;\n")
        if dword_bits <= 64:
            file.write(f"    using UL = uint{dword_bits}_t;\n")
        else:
            file.write(f"    using UL = __uint{dword_bits}_t;\n")
        file.write('\n')
        file.write(f"    constexpr static const std::size_t BITS = {params.bits};\n")
        file.write(f"    constexpr static const I M = {spec.modulus};\n")
        file.write(f"    constexpr static const I R2 = {params.square_montgomery_modulus};\n")
        file.write(f"    constexpr static const I RN = {params.montgomery_modulus};\n")
        zetas = str(params.zetas).replace('[', '{').replace(']', '}')
        file.write(f"    constexpr static const std::array<I, {len(params.zetas)}> ZETAS = {zetas};\n")
        file.write(spec.reduce.replace("_Q_", str(spec.modulus)))
        file.write("};\n")
        file.write('\n')
        file.write(f"// {spec.comment}\n")
        file.write(f"typedef IntegerRing<{spec.type_name}Params> {spec.type_name};\n")
        file.write('\n')
        file.write("#endif\n")

rings: list[RingSpec] = [
    RingSpec("dilithiumring.h", "2²³ - 2¹³ + 1", "DilithiumRing", 8380417, 256,
"""
    constexpr static I reduce(I x) {
        int32_t t((x + (1 << 22)) >> 23);
        return x - t * _Q_;
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 31) & _Q_);
    }
"""),
    RingSpec("fermat.h", "2¹⁶ + 1", "FermatRing", 65537, 512,
"""
    constexpr static I reduce(I x) {
        return (x & 0xFFFF) - (x >> 16);
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 31) & _Q_);
    }
"""),
    RingSpec("pervushin.h", "2⁶¹ - 1", "PervushinRing", 2305843009213693951, 1,
"""
    constexpr static I reduce(I x) {
        return (x & _Q_) + (x >> 61);
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 63) & _Q_);
    }
"""),
    RingSpec("solinas62.h", "2⁶² - 2⁸ - 2⁵ + 1", "Solinas62Ring", 0x3ffffffffffffee1, 16,
"""
    constexpr static I reduce(I x) {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * _Q_;
    }
    constexpr static I freeze(I x) {
        return x + ((x >> 63) & _Q_);
    }
""")
]

for ring in rings:
    bits = compute_bits(ring.modulus)
    word_bits = compute_word_bits(ring.modulus)
    square_montgomery_modulus = compute_square_montgomery_modulus(ring.modulus, word_bits)
    montgomery_modulus = compute_montgomery_modulus(ring.modulus, word_bits)
    montgomery_modulus = compute_centered_representation(montgomery_modulus, 2**word_bits)
    primitive_root_of_unity = compute_primitive_root_of_unity(ring.modulus, ring.cyclotomic_degree * 2)
    brv = [compute_bitreversal(i, log(ring.cyclotomic_degree, 2)) for i in range(0, ring.cyclotomic_degree)]
    zetas = [pow(primitive_root_of_unity, i, ring.modulus) for i in brv]
    zetas = [compute_montgomery_form(i, ring.modulus, montgomery_modulus, square_montgomery_modulus, word_bits) for i in zetas]
    zetas = [compute_centered_representation(i, ring.modulus) for i in zetas]
    params = RingParams(bits, word_bits, square_montgomery_modulus, montgomery_modulus, zetas)
    write_ring_cplusplus(ring, params)
