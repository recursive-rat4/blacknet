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
    meson_version : '>=1.5.0',
    version : '0.3.0.SNAPSHOT',
    license : 'LGPL-3.0-or-later',
    default_options : [
        'cpp_std=c++26',
        'b_ndebug=if-release',
    ],
)

desktop_feature = get_option('desktop')

fmt_dependency = dependency('fmt', version : '>=9.1.0')
spdlog_dependency = dependency('spdlog', version : '>=1.5.0')
boost_dependency = dependency('boost', version : '>=1.83.0')
qt_dependency = dependency('qt6', modules : 'Widgets', version : '>=6.2.5', required : desktop_feature)

desktop_feature = desktop_feature.require(qt_dependency.found(), error_message : 'requires Qt Framework')

if get_option('tests')
    boost_test_dependency = dependency('boost', modules : 'unit_test_framework', version : '>=1.67.0')
endif

if get_option('benchmarks')
    google_benchmark_dependency = dependency('benchmark', version : '>=1.1.0')
endif

compiler = meson.get_compiler('cpp')

# Current implementation of lattice cryptography largely relies on
# the signed integer overflow, which is undefined behaviour in stantard C++23
compiler.has_argument('-fwrapv', required : true)
add_project_arguments('-fwrapv', language : 'cpp')

config_data = configuration_data({
    'BLACKNET_OPTIMIZE' : get_option('optimization') in ['3', 's'],
    'BLACKNET_VERSION_STRING' : '"' + meson.project_version() + '"',
    'BLACKNET_HOST_SYSTEM' : '"' + host_machine.system() + '"',
    'BLACKNET_HOST_ARCH' : '"' + host_machine.cpu_family() + '"',
})
if host_machine.system() != 'windows'
    config_data.set('BLACKNET_HAVE_SYS_STAT', compiler.has_header('sys/stat.h'))
    config_data.set('BLACKNET_HAVE_SYS_UTSNAME', compiler.has_header('sys/utsname.h'))
    config_data.set('BLACKNET_HAVE_UNISTD', compiler.has_header('unistd.h'))
endif
if host_machine.system() not in ['windows', 'darwin']
    config_data.set('BLACKNET_HAVE_FDATASYNC', compiler.has_header_symbol('unistd.h', 'fdatasync'))
    config_data.set('BLACKNET_HAVE_GETENTROPY', compiler.has_header_symbol('unistd.h', 'getentropy'))
endif
if host_machine.system() == 'windows'
    config_data.set('BLACKNET_HAVE_FILEAPI', compiler.has_header('fileapi.h'))
    config_data.set('BLACKNET_HAVE_NTSECAPI', compiler.has_header('ntsecapi.h'))
endif
if host_machine.system() == 'darwin'
    config_data.set('BLACKNET_HAVE_SYS_RANDOM', compiler.has_header('sys/random.h'))
    config_data.set('BLACKNET_HAVE_FULLFSYNC', compiler.has_header_symbol('fcntl.h', 'F_FULLFSYNC'))
    config_data.set('BLACKNET_HAVE_GETENTROPY', compiler.has_header_symbol('sys/random.h', 'getentropy'))
endif
configure_file(
    output : 'blacknet-config.h',
    configuration : config_data,
)
config_includes = include_directories('.')

subdir('compat')
subdir('crypto')
subdir('log')
if desktop_feature.allowed()
    subdir('desktop')
endif
