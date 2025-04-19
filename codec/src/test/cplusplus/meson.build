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
    codec_test_arguments = ['--log_format=JUNIT', '--log_sink=TEST-codec.xml']
else
    codec_test_arguments = []
endif

codec_test_sources = [
    'base64.cpp',
    'test.cpp',
]
codec_test_includes = [
    codec_main_includes,
    include_directories('.')
]
codec_test_dependencies = [
    codec_main_dependencies,
    boost_test_dependency
]
codec_test_bin = executable(
    'codec_test',
    codec_test_sources,
    implicit_include_directories : false,
    include_directories : codec_test_includes,
    dependencies : codec_test_dependencies
)
test(
    'codec test',
    codec_test_bin,
    args : codec_test_arguments,
)
