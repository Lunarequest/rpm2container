mod cli;
mod config;
mod manifest;
mod utils;

use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::{fs::File, io::Write};

use anyhow::Result;
use clap::Parser;
use config::Config;

use cli::{Cli, CliCommand};
use manifest::Manifest;
use utils::{pack::pack, rootfs::init_rootfs};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        CliCommand::Build { path } => {
            let file = File::open(path)?;
            let mainfest_read: Manifest = serde_yaml::from_reader(file)?;
            let mut args = vec![mainfest_read.entrypoint.command.clone()];
            match &mainfest_read.entrypoint.args {
                Some(process_args) => {
                    let mut process_args = process_args.clone();
                    args.append(&mut process_args);
                    drop(process_args);
                }
                None => {}
            }

            let env = match mainfest_read.entrypoint.env.clone() {
                Some(env) => env,
                None => vec![],
            };

            let root = PathBuf::from("/tmp/bundle");
            create_dir_all(&root)?;
            //create a rootfs
            init_rootfs(&root.to_path_buf(), mainfest_read)?;
            let config = serde_json::to_string(&Config::new(env, args))?;
            let mut config_file = File::create(&root.join("config.json"))?;
            config_file.write_all(config.as_bytes())?;
            pack()?;
            remove_dir_all("/tmp/bundle")?;
        }
    }
    Ok(())
}
