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
    io_test_arguments = ['--log_format=JUNIT', '--log_sink=TEST-io.xml']
else
    io_test_arguments = []
endif

io_test_sources = [
    'hash_output_stream.cpp',
    'size_output_stream.cpp',
    'span_output_stream.cpp',
    'test.cpp',
]
io_test_includes = [
    io_main_includes,
    crypto_main_includes,
]
io_test_dependencies = [
    io_main_dependencies,
    crypto_main_dependencies,
    boost_test_dependency
]
io_test_bin = executable(
    'io_test',
    io_test_sources,
    implicit_include_directories : false,
    include_directories : io_test_includes,
    dependencies : io_test_dependencies
)
test(
    'io test',
    io_test_bin,
    args : io_test_arguments,
)
