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

if get_option('tests_report_junit')
    crypto_test_arguments = ['--log_format=JUNIT', '--log_sink=TEST-crypto.xml']
else
    crypto_test_arguments = []
endif

crypto_test_sources = [
    'ajtaicommitment.cpp',
    'bfv.cpp',
    'bigint.cpp',
    'blacklemon.cpp',
    'ccsbuilder.cpp',
    'chacha20.cpp',
    'customizableconstraintsystem.cpp',
    'dilithium.cpp',
    'eqextension.cpp',
    'fastrng.cpp',
    'fermat.cpp',
    'fs.cpp',
    'hypercube.cpp',
    'interpolation.cpp',
    'jive.cpp',
    'latticefold.cpp',
    'matrixsparse.cpp',
    'matrix.cpp',
    'merkletree.cpp',
    'module.cpp',
    'multilinearextension.cpp',
    'pastacurves.cpp',
    'pedersencommitment.cpp',
    'pervushin.cpp',
    'poseidon2.cpp',
    'powextension.cpp',
    'r1cs.cpp',
    'ringproduct.cpp',
    'semigroup.cpp',
    'sha2.cpp',
    'sha3.cpp',
    'siphash.cpp',
    'solinas62.cpp',
    'sponge.cpp',
    'sumcheck.cpp',
    'test.cpp',
    'univariatepolynomial.cpp',
    'vector.cpp',
]
crypto_test_includes = [crypto_main_includes, include_directories('.')]
crypto_test_dependencies = [crypto_main_dependencies, boost_test_dependency]
crypto_test_bin = executable(
    'crypto_test',
    crypto_test_sources,
    implicit_include_directories : false,
    include_directories : crypto_test_includes,
    dependencies : crypto_test_dependencies
)
test(
    'crypto test',
    crypto_test_bin,
    args : crypto_test_arguments,
)
