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

mod client;

use crate::client::Client;
use blacknet_compat::{
    config::Config,
    {XDGDirectories, mode},
};
use clap::{Parser, Subcommand};
use core::error::Error;
use serde_json::{Value, from_str, to_writer_pretty};
use std::{io::stdout, process::ExitCode};

#[derive(Parser)]
#[command(version)]
#[command(about = "Blacknet RPC client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// RPC command.
#[derive(Subcommand)]
enum Command {
    /// Node info.
    Node,
    /// Peer info.
    Peers,
    /// Staking info.
    Staking {
        /// If not specified returns total of all node's wallets.
        address: Option<String>,
    },
    /// TxPool info.
    Txpool,
}

fn cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mode = mode()?;
    let dirs = XDGDirectories::new(mode.subdirectory())?;
    let config = Config::load(dirs.config())?;

    let client = Client::new(&config.rpc.bind.host, config.rpc.bind.port)?;

    let reply = match cli.command {
        Command::Node => client.get("/api/v2/node"),
        Command::Peers => client.get("/api/v2/peers"),
        Command::Staking { address: None } => client.get("/api/v2/staking"),
        Command::Staking {
            address: Some(address),
        } => client.get(&format!("/api/v2/staking/{}", address)),
        Command::Txpool => client.get("/api/v2/txpool"),
    }?;

    if let Ok(json) = from_str::<Value>(&reply) {
        to_writer_pretty(stdout(), &json)?;
        println!()
    } else {
        println!("{reply}")
    }

    Ok(())
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
