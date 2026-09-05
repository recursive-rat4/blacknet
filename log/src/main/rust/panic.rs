/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::error;
use core::fmt::Write;
use spdlog::Logger;
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    sync::atomic::{AtomicBool, Ordering},
};

static SHOWN_NOTE: AtomicBool = AtomicBool::new(false);

pub fn set_panic_hook(logger: Logger) {
    std::panic::set_hook(Box::new(move |info| {
        let mut msg = String::new();

        let thread = std::thread::current();
        if let Some(thread_name) = thread.name() {
            let _ = write!(msg, "In {thread_name}");
        } else {
            let _ = write!(msg, "In {:?}", thread.id());
        }

        if let Some(location) = info.location() {
            let _ = write!(msg, " at {location}");
        }

        if let Some(payload) = info.payload_as_str() {
            let _ = write!(msg, ": {payload}");
        }

        let backtrace = Backtrace::capture();
        match backtrace.status() {
            BacktraceStatus::Captured => {
                let _ = write!(msg, "\n{backtrace}");
            }
            BacktraceStatus::Disabled => {
                if !SHOWN_NOTE.swap(true, Ordering::Relaxed) {
                    let _ = writeln!(
                        msg,
                        "\nNote: run with RUST_LIB_BACKTRACE=1 environment variable for backtraces"
                    );
                }
            }
            BacktraceStatus::Unsupported => {
                if !SHOWN_NOTE.swap(true, Ordering::Relaxed) {
                    let _ = writeln!(msg, "\nNote: backtraces are unsupported in this build");
                }
            }
            status => {
                if !SHOWN_NOTE.swap(true, Ordering::Relaxed) {
                    let _ = writeln!(msg, "\nNote: unknown backtrace status {status:?}");
                }
            }
        }

        error!(logger, "{msg}");
    }));
}
