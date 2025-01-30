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

subdir('main/cplusplus')

if get_option('tests')
    subdir('test/cplusplus')
endif

if get_option('benchmarks')
    subdir('benchmark/cplusplus')
endif
