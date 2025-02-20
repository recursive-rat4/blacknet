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

crypto_test_sources = [
    'ajtaicommitment.cpp',
    'bfv.cpp',
    'bigint.cpp',
    'blacklemon.cpp',
    'ccsbuilder.cpp',
    'customizableconstraintsystem.cpp',
    'dilithium.cpp',
    'eqextension.cpp',
    'fermat.cpp',
    'hypercube.cpp',
    'latticefold.cpp',
    'matrixsparse.cpp',
    'matrix.cpp',
    'multilinearextension.cpp',
    'pastacurves.cpp',
    'pedersencommitment.cpp',
    'pervushin.cpp',
    'poseidon2.cpp',
    'powextension.cpp',
    'r1cs.cpp',
    'ringproduct.cpp',
    'semigroup.cpp',
    'solinas62.cpp',
    'sponge.cpp',
    'sumcheck.cpp',
    'test.cpp',
    'univariatepolynomial.cpp',
    'vector.cpp',
]
crypto_test_dependencies = boost_test_dependency
crypto_test_bin = executable(
    'crypto_test',
    crypto_test_sources,
    implicit_include_directories : false,
    include_directories : crypto_main_includes,
    dependencies : crypto_test_dependencies
)
test('crypto test', crypto_test_bin)
