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
use clap::Parser;
use std::error::Error;
use std::process::ExitCode;

#[derive(Parser)]
#[command(version)]
#[command(about = "Blacknet RPC client", long_about = None)]
struct Cli {
    /// RPC command.
    command: String,
    /// Arguments for the command.
    args: Vec<String>,
}

#[expect(unused_variables)]
fn cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mode = mode()?;
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
