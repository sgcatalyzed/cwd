mod debug;
mod genesis;
mod init;
mod keys;
mod query;
mod reset;
mod start;
mod tendermint;
mod tx;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use cwd::path;
use tracing::error;
use tracing_subscriber::filter::LevelFilter;

use crate::{
    debug::DebugCmd, genesis::GenesisCmd, init::InitCmd, keys::KeysCmd, query::QueryCmd,
    reset::ResetCmd, start::StartCmd, tendermint::TendermintCmd, tx::TxCmd,
};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Application home directory
    #[arg(long)]
    pub home: Option<PathBuf>,

    /// Increase output logging verbosity to DEBUG level
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Helper command useful for developers
    Debug(DebugCmd),

    /// Utilities for preparing the genesis state
    Genesis(GenesisCmd),

    /// Initialize application home directory
    Init(InitCmd),

    /// Manage private keys
    Keys(KeysCmd),

    /// Query the application state
    #[command(alias = "q")]
    Query(QueryCmd),

    /// Start the ABCI server
    Start(StartCmd),

    /// Query Tendermint RPC
    Tendermint(TendermintCmd),

    /// Sign and broadcast transactions
    Tx(TxCmd),

    /// Delete the local application data
    UnsafeResetAll(ResetCmd),
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // set home directory
    let home_dir = match &cli.home {
        Some(home) => home.clone(),
        None => path::default_app_home()?,
    };

    // set log level
    let log_level = if cli.debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    match cli.command {
        Command::Debug(cmd) => cmd.run(),
        Command::Genesis(cmd) => cmd.run(),
        Command::Init(cmd) => cmd.run(&home_dir),
        Command::Keys(cmd) => cmd.run(&home_dir),
        Command::Query(cmd) => cmd.run(&home_dir).await,
        Command::Start(cmd) => cmd.run(&home_dir),
        Command::Tendermint(cmd) => cmd.run(&home_dir).await,
        Command::Tx(cmd) => cmd.run(&home_dir).await,
        Command::UnsafeResetAll(cmd) => cmd.run(&home_dir),
    }
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        error!("command failed with error: {}", err);
    }
}

type Result<T> = core::result::Result<T, cwd::Error>;
