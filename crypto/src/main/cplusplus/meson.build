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

compiler = meson.get_compiler('cpp')
compiler.has_argument('-fwrapv', required : true)
add_project_arguments('-fwrapv', language : 'cpp')
compiler.has_type('__int128_t', required : true)
compiler.has_type('__uint128_t', required : true)

if get_option('optimization') == '3' or get_option('optimization') == 's'
    add_project_arguments('-DBLACKNET_CRYPTO_OMG_OPTIMIZE', language : 'cpp')
endif

add_project_dependencies(
    dependency('boost', modules : ['random'], version : '>=1.47.0'),
    language : 'cpp'
)

crypto_main_includes = include_directories('.')
