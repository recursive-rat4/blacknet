#!/usr/bin/env sage
#
# Copyright (c) 2024-2025 Pavel Vasin
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

from typing import NamedTuple, Optional
from sage.rings.integer import Integer
from sympy.ntheory.residue_ntheory import nthroot_mod

class CyclotomicRingSpec(NamedTuple):
    cyclotomic_index: int
    inertia_degree: Optional[int]

class IntegerRingSpec(NamedTuple):
    file_name: str
    comment: str
    type_name: str
    modulus: int
    extension: Optional[CyclotomicRingSpec]
    reduce: str

class IntegerRingParams(NamedTuple):
    bits: int
    word_bits: int
    division_ring: bool
    square_montgomery_modulus: int
    montgomery_modulus: int
    twiddles: Optional[list[int]]
    inv_ntt_scale: Optional[int]

def compute_bits(number):
    return ceil(log(number, 2))

def compute_word_bits(number):
    bits = compute_bits(number)
    return 2**(ceil(log(bits, 2)))

def compute_division_ring(number):
    return is_prime(number)

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
        file.write('namespace blacknet::crypto {\n')
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
        if params.division_ring:
            file.write("    constexpr static const bool is_division_ring = true;\n")
        else:
            file.write("    constexpr static const bool is_division_ring = false;\n")
        file.write('\n')
        file.write(f"    constexpr static const std::size_t BITS = {params.bits};\n")
        file.write(f"    constexpr static const I M = {spec.modulus};\n")
        file.write(f"    constexpr static const I R2 = {params.square_montgomery_modulus};\n")
        file.write(f"    constexpr static const I RN = {params.montgomery_modulus};\n")
        if params.twiddles != None:
            twiddles = str(params.twiddles).replace('[', '{').replace(']', '}')
            file.write(f"    constexpr static const std::array<I, {len(params.twiddles)}> TWIDDLES = {twiddles};\n")
        if params.inv_ntt_scale != None:
            file.write(f"    constexpr static const I INVERSE_TWIDDLES = {params.inv_ntt_scale};\n")
        file.write(spec.reduce.replace("_Q_", str(spec.modulus)))
        file.write("};\n")
        file.write('\n')
        file.write(f"// {spec.comment}\n")
        file.write(f"typedef IntegerRing<{spec.type_name}Params> {spec.type_name};\n")
        file.write('\n')
        file.write("}\n")
        file.write('\n')
        file.write("#endif\n")

rings: list[IntegerRingSpec] = [
    IntegerRingSpec("dilithiumring.h", "2²³ - 2¹³ + 1", "DilithiumRing", 8380417,
    CyclotomicRingSpec(512, None),
"""
    constexpr static I reduce(I x) {
        int32_t t((x + (1 << 22)) >> 23);
        return x - t * _Q_;
    }
"""),
    IntegerRingSpec("fermat.h", "2¹⁶ + 1", "FermatRing", 65537,
    CyclotomicRingSpec(2048, None),
"""
    constexpr static I reduce(I x) {
        return (x & 0xFFFF) - (x >> 16);
    }
"""),
    IntegerRingSpec("pervushin.h", "2⁶¹ - 1", "PervushinRing", 2305843009213693951,
    None,
"""
    constexpr static I reduce(I x) {
        return (x & _Q_) + (x >> 61);
    }
"""),
    IntegerRingSpec("solinas62.h", "2⁶² - 2⁸ - 2⁵ + 1", "Solinas62Ring", 0x3ffffffffffffee1,
    CyclotomicRingSpec(128, 4),
"""
    constexpr static I reduce(I x) {
        int32_t t((x + (1l << 61)) >> 62);
        return x - t * _Q_;
    }
""")
]

for ring in rings:
    bits = compute_bits(ring.modulus)
    word_bits = compute_word_bits(ring.modulus)
    division_ring = compute_division_ring(ring.modulus)
    square_montgomery_modulus = compute_square_montgomery_modulus(ring.modulus, word_bits)
    montgomery_modulus = compute_montgomery_modulus(ring.modulus, word_bits)
    montgomery_modulus = compute_centered_representation(montgomery_modulus, 2**word_bits)
    if ring.extension != None:
        extension = ring.extension
        assert is_power_of_two(extension.cyclotomic_index), "Non-power-of-two cyclotomic rings are not supported"
        cyclotomic_polynomial_degree = euler_phi(extension.cyclotomic_index)
        if extension.inertia_degree != None:
            assert cyclotomic_polynomial_degree % extension.inertia_degree == 0, "Non-integer split"
            split = cyclotomic_polynomial_degree / extension.inertia_degree
            assert ring.modulus % (4 * split) == 1 + 2 * split, "Ideal is insufficiently inert"
            split = Integer(split)
        else:
            split = cyclotomic_polynomial_degree
        primitive_root_of_unity = compute_primitive_root_of_unity(ring.modulus, 2 * split)
        brv = [compute_bitreversal(i, log(split, 2)) for i in range(0, split)]
        twiddles = [pow(primitive_root_of_unity, i, ring.modulus) for i in brv]
        twiddles = [compute_montgomery_form(i, ring.modulus, montgomery_modulus, square_montgomery_modulus, word_bits) for i in twiddles]
        twiddles = [compute_centered_representation(i, ring.modulus) for i in twiddles]
        inv_ntt_scale = pow(split, -1, ring.modulus)
        inv_ntt_scale = compute_montgomery_form(inv_ntt_scale, ring.modulus, montgomery_modulus, square_montgomery_modulus, word_bits)
        inv_ntt_scale = compute_centered_representation(inv_ntt_scale, ring.modulus)
    else:
        twiddles = None
        inv_ntt_scale = None
    params = IntegerRingParams(bits, word_bits, division_ring, square_montgomery_modulus, montgomery_modulus, twiddles, inv_ntt_scale)
    write_ring_cplusplus(ring, params)
