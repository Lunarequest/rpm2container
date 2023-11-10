mod cli;
mod json;
mod manifest;
mod utils;

use std::{
    fs::{copy, create_dir_all, remove_dir_all, remove_file, File},
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use flate2::read::GzDecoder;

use cli::{Cli, CliCommand};
use json::{index::Index, info::Info, layers::Layer, manifest::ManifestOCI};
use manifest::Manifest;
use utils::{
    pack::{pack_bundle, pack_layer},
    rootfs::init_rootfs,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        CliCommand::Build { path, image } => {
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
            let mut env = vec![
                "PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin".to_string(),
            ];
            if let Some(env_user) = mainfest_read.entrypoint.env.clone() {
                let mut env_user = env_user.clone();
                env.append(&mut env_user);
            };

            let root = PathBuf::from("/tmp/bundle");
            create_dir_all(&root)?;
            //create a rootfs
            init_rootfs(&root.to_path_buf(), mainfest_read)?;
            pack_layer()?;

            let mut buf: Vec<u8> = vec![];
            let mut bundle = File::open("./bundle.tar.gz")?;
            bundle.read_to_end(&mut buf)?;

            let mut decoder = GzDecoder::new(Cursor::new(buf));
            let mut uncompressed_data = Vec::new();

            decoder.read_to_end(&mut uncompressed_data)?;

            let hash = sha256::digest(uncompressed_data);
            let layer_size = bundle.metadata()?.len();
            create_dir_all("/tmp/out")?;
            copy("bundle.tar.gz", format!("/tmp/out/{hash}.tar.gz"))?;
            remove_file("bundle.tar.gz")?;

            let layer = Layer::new(&hash, args, env);
            let layer_out = serde_json::to_string(&layer)?;
            let layer_digest = sha256::digest(&layer_out);
            let mut layer_file = File::create(format!("/tmp/out/sha256:{layer_digest}"))?;
            layer_file.write_all(layer_out.as_bytes())?;
            let config_size = layer_file.metadata()?.len();

            let config = Info::new(config_size, &layer_digest, layer_size, &hash);
            let config_out = serde_json::to_string(&config)?;
            let config_digest = sha256::digest(&config_out);
            let mut config_file = File::create(format!("/tmp/out/sha256:{config_digest}"))?;
            config_file.write_all(config_out.as_bytes())?;
            let config_size = config_file.metadata()?.len();

            let index = Index::new(config_size, &config_digest);
            let index_out = serde_json::to_string(&index)?;
            let mut index_file = File::create("/tmp/out/index.json")?;
            index_file.write_all(index_out.as_bytes())?;

            let manifest = vec![ManifestOCI::new(
                format!("sha256:{layer_digest}"),
                image,
                format!("{hash}.tar.gz"),
            )];

            let manifest_out = serde_json::to_string(&manifest)?;
            let mut manifest_file = File::create("/tmp/out/manifest.json")?;
            manifest_file.write_all(manifest_out.as_bytes())?;
            pack_bundle()?;
            remove_dir_all("/tmp/out")?;
            remove_dir_all("/tmp/bundle")?;
        }
    }
    Ok(())
}
