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

project('blacknet crypto', 'cpp',
    meson_version : '>=1.3.0',
    version : '0.3.0.SNAPSHOT',
    license : 'LGPL-3.0-or-later',
    default_options : [
        'cpp_std=c++26',
        'b_ndebug=if-release',
    ],
)

assert(meson.get_compiler('cpp').has_argument('-fwrapv'), 'Unsupported C++ compiler')
add_project_arguments('-fwrapv', language : 'cpp')

if get_option('optimization') == '3' or get_option('optimization') == 's'
    add_project_arguments('-DBLACKNET_CRYPTO_OMG_OPTIMIZE', language : 'cpp')
endif

add_project_dependencies(dependency('boost', modules: ['random']), language : 'cpp')

if get_option('tests')
    test_sources = [
        'ajtaicommitmenttest.cpp',
        'bfvtest.cpp',
        'biginttest.cpp',
        'blacklemontest.cpp',
        'ccsbuildertest.cpp',
        'customizableconstraintsystemtest.cpp',
        'dilithiumtest.cpp',
        'eqextensiontest.cpp',
        'fermattest.cpp',
        'hypercubetest.cpp',
        'latticefoldtest.cpp',
        'matrixsparsetest.cpp',
        'matrixtest.cpp',
        'multilinearextensiontest.cpp',
        'pastacurvestest.cpp',
        'pedersencommitmenttest.cpp',
        'pervushintest.cpp',
        'poseidon2test.cpp',
        'powextensiontest.cpp',
        'r1cstest.cpp',
        'ringproducttest.cpp',
        'semigrouptest.cpp',
        'solinas62test.cpp',
        'spongetest.cpp',
        'sumchecktest.cpp',
        'test.cpp',
        'univariatepolynomialtest.cpp',
        'vectortest.cpp',
    ]
    test_dependencies = dependency('boost', modules: ['unit_test_framework'])
    test_bin = executable('crypto_test', test_sources, dependencies : test_dependencies)
    test('crypto test', test_bin)
endif

if get_option('benchmarks')
    benchmark_sources = [
        'ajtaicommitmentbenchmark.cpp',
        'benchmark.cpp',
        'cyclotomicringbenchmark.cpp',
        'extensionfieldbenchmark.cpp',
        'integerringbenchmark.cpp',
        'latticefoldbenchmark.cpp',
        'pastacurvesbenchmark.cpp',
        'pedersencommitmentbenchmark.cpp',
        'poseidon2benchmark.cpp',
        'primefieldbenchmark.cpp',
        'sumfoldbenchmark.cpp',
    ]
    benchmark_dependencies = dependency('benchmark')
    benchmark_bin = executable('crypto_benchmark', benchmark_sources, dependencies : benchmark_dependencies)
    benchmark('crypto benchmark', benchmark_bin)
endif
