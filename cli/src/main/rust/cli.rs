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

use blacknet_compat::{XDGDirectories, mode};
use std::env::args;
use std::error::Error;
use std::process::ExitCode;

fn cli() -> Result<(), Box<dyn Error>> {
    if args().nth(1).is_some_and(|arg| arg == "--version") {
        println!("Blacknet Daemon {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mode = mode()?;
    #[expect(unused_variables)]
    let dirs = XDGDirectories::new(mode.subdirectory())?;
    todo!();
}

fn main() -> ExitCode {
    match cli() {
        Ok(..) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::FAILURE
        }
    }
}
