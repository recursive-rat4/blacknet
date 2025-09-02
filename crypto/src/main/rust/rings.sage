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

from typing import NamedTuple
from sage.rings.integer import Integer
from sympy.ntheory.residue_ntheory import nthroot_mod

class TwiddleSpec(NamedTuple):
    cyclotomic_index: int
    inertia_degree: int

class IntegerRingSpec(NamedTuple):
    import_string: str
    type_name: str
    modulus: int
    extensions: list[TwiddleSpec]

class TwiddleParam(NamedTuple):
    twiddles: list[int]
    inv_ntt_scale: int

class IntegerRingParam(NamedTuple):
    bits: int
    extensions: list[TwiddleParam]

def compute_bits(number):
    return ceil(log(number, 2))

def compute_centered_representation(number, modulus):
    if number > modulus / 2:
        return number - modulus
    if number < - modulus / 2:
        return number + modulus
    return number

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

def write_rings_rust():
    with open("twiddles.rs", 'w', encoding="utf-8") as file:
        file.write("// Auto-generated with rings.sage\n")
        file.write('\n')
        file.write('#![cfg_attr(rustfmt, rustfmt_skip)]\n')
        file.write('\n')
        file.write("use crate::numbertheoretictransform::Twiddles;\n")
        for spec, param in zip(ring_specs, ring_params):
            file.write('\n')
            file.write(f"use {spec.import_string};\n")
            file.write('\n')
            for extension in param.extensions:
                file.write(f"impl Twiddles<{len(extension.twiddles)}> for {spec.type_name}" + " {\n")
                file.write(f"    const TWIDDLES: [Self; {len(extension.twiddles)}] = [\n")
                for twiddle in extension.twiddles:
                    file.write("        unsafe { " + f"Self::from_unchecked({twiddle})" + " },\n")
                file.write("    ];\n")
                file.write("    const SCALE: Self = unsafe { " + f"Self::from_unchecked({extension.inv_ntt_scale})" + " };\n")
                file.write("}\n")

ring_specs: list[IntegerRingSpec] = [
    IntegerRingSpec("crate::fermat::FermatField", "FermatField", 65537,
        [
            TwiddleSpec(2048, 1),
        ]
    ),
    IntegerRingSpec("crate::lm::LMField", "LMField", 1152921504606847009,
        [
            TwiddleSpec(128, 4),
        ]
    ),
]

ring_params: list[IntegerRingParam] = []

for ring in ring_specs:
    bits = compute_bits(ring.modulus)
    extensions = []
    for extension in ring.extensions:
        assert is_power_of_two(extension.cyclotomic_index), "Non-power-of-two cyclotomic rings are not supported"
        cyclotomic_polynomial_degree = euler_phi(extension.cyclotomic_index)
        assert cyclotomic_polynomial_degree % extension.inertia_degree == 0, "Non-integer split"
        split = Integer(cyclotomic_polynomial_degree / extension.inertia_degree)
        primitive_root_of_unity = compute_primitive_root_of_unity(ring.modulus, 2 * split)
        brv = [compute_bitreversal(i, log(split, 2)) for i in range(0, split)]
        twiddles = [pow(primitive_root_of_unity, i, ring.modulus) for i in brv]
        twiddles = [compute_centered_representation(i, ring.modulus) for i in twiddles]
        inv_ntt_scale = pow(split, -1, ring.modulus)
        inv_ntt_scale = compute_centered_representation(inv_ntt_scale, ring.modulus)
        param = TwiddleParam(twiddles, inv_ntt_scale)
        extensions.append(param)
    param = IntegerRingParam(bits, extensions)
    ring_params.append(param)

write_rings_rust()
