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

project('blacknet', 'cpp',
    meson_version : '>=1.3.0',
    version : '0.3.0.SNAPSHOT',
    license : 'LGPL-3.0-or-later',
    default_options : [
        'cpp_std=c++26',
        'b_ndebug=if-release',
    ],
)

boost_random_dependency = dependency('boost', modules : ['random'], version : '>=1.47.0')

if get_option('tests')
    boost_test_dependency = dependency('boost', modules: ['unit_test_framework'], version : '>=1.67.0')
endif

if get_option('benchmarks')
    google_benchmark_dependency = dependency('benchmark', version : '>=1.1.0')
endif

subdir('crypto')
subdir('vm')
