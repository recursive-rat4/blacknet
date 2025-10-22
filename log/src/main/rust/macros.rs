/*
 * Copyright (c) 2025 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#[macro_export]
macro_rules! critical {
    ($logger:expr, $($arg:tt)+) => {
        spdlog::log!(logger: $logger, spdlog::Level::Critical, $($arg)+)
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)+) => {
        spdlog::log!(logger: $logger, spdlog::Level::Error, $($arg)+)
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $($arg:tt)+) => {
        spdlog::log!(logger: $logger, spdlog::Level::Warn, $($arg)+)
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)+) => {
        spdlog::log!(logger: $logger, spdlog::Level::Info, $($arg)+)
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)+) => {
        spdlog::log!(logger: $logger, spdlog::Level::Debug, $($arg)+)
    };
}

#[macro_export]
macro_rules! trace {
    ($logger:expr, $($arg:tt)+) => {
        spdlog::log!(logger: $logger, spdlog::Level::Trace, $($arg)+)
    };
}
