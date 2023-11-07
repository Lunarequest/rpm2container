mod cli;
mod manifest;
mod utils;

use std::fs::File;

use anyhow::Result;
use clap::Parser;
use tempfile::TempDir;

use cli::{Cli, CliCommand};
use manifest::Manifest;
use utils::rootfs::init_rootfs;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        CliCommand::Build { path } => {
            let file = File::open(path)?;
            let mainfest_read: Manifest = serde_yaml::from_reader(file)?;

            let root = TempDir::with_prefix("rpm2container-")?;
            init_rootfs(&root.path().to_path_buf(), mainfest_read)?;
        }
    }
    Ok(())
}
