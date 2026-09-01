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
use core::{error::Error, num::NonZero};
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
    /// Peering.
    #[command(subcommand)]
    Peer(Peer),
    /// Staking info.
    Staking {
        /// If not specified returns total of all node's wallets.
        address: Option<String>,
    },
    /// TxPool queries.
    #[command(subcommand)]
    Txpool(Txpool),
    /// Database queries.
    #[command(subcommand)]
    Db(Db),
    /// Send raw transaction.
    SendRawTransaction {
        /// Hex-encoded transaction bytes.
        hex: String,
    },
    /// Make bootstrap file (slow).
    MakeBootstrap,
    /// Shut down node.
    Shutdown,
    /// Debug info.
    #[command(subcommand, hide = true)]
    Debug(Debug),
}

/// Peering command.
#[derive(Subcommand)]
enum Peer {
    /// List connected peers.
    List,
    /// Connect to a peer.
    Connect {
        /// Network address.
        address: String,
        /// If not specified tries default port.
        port: Option<u16>,
    },
    /// Disconnect a peer.
    Disconnect {
        /// Node's peer id.
        id: NonZero<u64>,
    },
}

/// TxPool command.
#[derive(Subcommand)]
enum Txpool {
    /// List transaction hashes.
    List,
    /// Transaction by hash.
    Transaction {
        /// Hash of transaction.
        hash: String,
        /// If true returns hex-encoded transaction bytes.
        raw: Option<bool>,
    },
}

/// Database command.
#[derive(Subcommand)]
enum Db {
    /// Block by hash.
    Block {
        /// Hash of block.
        hash: String,
        /// If true includes transactions.
        detail: Option<bool>,
    },
    /// Block index by hash.
    BlockIndex {
        /// Hash of block.
        hash: String,
    },
    /// Account by address.
    Account {
        /// Address of account.
        address: String,
        /// If not specified uses default number.
        confirmations: Option<u32>,
    },
    /// Block hash by height (slow).
    BlockHash {
        /// Height of block.
        height: u32,
    },
}

/// Debug command.
#[derive(Subcommand)]
enum Debug {
    /// Fjall database counters.
    Fjall,
    /// Tokio runtime metrics.
    Tokio,
    /// Check block database (slow).
    Blockdb,
    /// Check coin database (slow).
    Coindb,
}

fn cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mode = mode()?;
    let dirs = XDGDirectories::new(mode.subdirectory())?;
    let config = Config::load(dirs.config())?;

    let client = Client::new(&config.rpc.bind.host, config.rpc.bind.port)?;

    let reply = match cli.command {
        Command::Node => client.get("/api/v2/node"),
        Command::Peer(Peer::List) => client.get("/api/v2/peers"),
        Command::Peer(Peer::Connect { address, port }) => client.get(&if let Some(port) = port {
            format!("/api/v2/addpeer/{address}/{port}")
        } else {
            format!("/api/v2/addpeer/{address}")
        }),
        Command::Peer(Peer::Disconnect { id }) => {
            client.get(&format!("/api/v2/disconnectpeer/{id}"))
        }
        Command::Staking { address: None } => client.get("/api/v2/staking"),
        Command::Staking {
            address: Some(address),
        } => client.get(&format!("/api/v2/staking/{address}")),
        Command::Txpool(Txpool::List) => client.get("/api/v2/txpool"),
        Command::Txpool(Txpool::Transaction { hash, raw }) => client.get(&if let Some(raw) = raw {
            format!("/api/v2/txpool/transaction/{hash}/{raw}")
        } else {
            format!("/api/v2/txpool/transaction/{hash}")
        }),
        Command::Db(Db::Block { hash, detail }) => client.get(&if let Some(txdetail) = detail {
            format!("/api/v2/block/{hash}/{txdetail}")
        } else {
            format!("/api/v2/block/{hash}")
        }),
        Command::Db(Db::BlockIndex { hash }) => client.get(&format!("/api/v2/blockindex/{hash}")),
        Command::Db(Db::Account {
            address,
            confirmations,
        }) => client.get(&if let Some(confirmations) = confirmations {
            format!("/api/v2/account/{address}/{confirmations}")
        } else {
            format!("/api/v2/account/{address}")
        }),
        Command::Db(Db::BlockHash { height }) => client.get(&format!("/api/v2/blockhash/{height}")),
        Command::SendRawTransaction { hex } => {
            client.get(&format!("/api/v2/sendrawtransaction/{hex}"))
        }
        Command::MakeBootstrap => client.get("/api/v2/makebootstrap"),
        Command::Shutdown => client.get("/api/shutdown"),
        Command::Debug(Debug::Fjall) => client.get("/api/v2/leveldb/stats"),
        Command::Debug(Debug::Tokio) => client.get("/api/debug/tokio/metrics"),
        Command::Debug(Debug::Blockdb) => client.get("/api/v2/blockdb/check"),
        Command::Debug(Debug::Coindb) => client.get("/api/v2/ledger/check"),
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
