use std::path::PathBuf;

use clap::{Parser, Subcommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[clap(author="Luna D Dragon",
        version=VERSION,
        about="A simple tool to make from scratch containerss",
        long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum CliCommand {
    Build {
        #[clap(short = 'i', long = "image")]
        image: String,
        path: PathBuf,
    },
}
