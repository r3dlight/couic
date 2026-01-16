use clap::{Parser, Subcommand};

use client::{CouicClient, CouicError};

use crate::VERSION;
use crate::config::ConfigError;

mod clients;
mod policy;
mod sets;
mod stats;

use policy::{DropSubCommand, IgnoreSubCommand, PolicyCommand};

#[derive(Parser, Debug)]
#[command(name = "couicctl")]
#[command(about = "Control couic firewall", long_about = None)]
#[command(version = VERSION)]
pub struct Cli {
    /// Path to config file
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/couic/couicctl.toml"
    )]
    pub config: String,
    #[arg(long, hide = true)]
    pub markdown_help: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Couic error: {0}")]
    Couic(#[from] CouicError),
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),
    #[error("RIPE API error: {0}")]
    Ripe(#[from] crate::ripe::RipeError),
    #[error("Generic error: {0}")]
    Generic(String),
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Clients(clients::ClientsCommand),
    Stats(stats::StatsCommand),
    Sets(sets::SetsCommand),
    Drop(PolicyCommand<DropSubCommand>),
    Ignore(PolicyCommand<IgnoreSubCommand>),
}

pub fn execute(mut client: CouicClient, command: Commands) -> Result<(), CommandError> {
    match command {
        Commands::Clients(cmd) => cmd.execute(&mut client),
        Commands::Stats(cmd) => cmd.execute(&mut client),
        Commands::Sets(cmd) => cmd.execute(&mut client),
        Commands::Drop(cmd) => cmd.execute(&mut client),
        Commands::Ignore(cmd) => cmd.execute(&mut client),
    }
}

pub trait Command {
    fn execute(&self, client: &mut CouicClient) -> Result<(), CommandError>;
}
