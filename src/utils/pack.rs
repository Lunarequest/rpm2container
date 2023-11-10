use anyhow::{anyhow, Result};
use std::{
    fs::{copy, create_dir, remove_file, File},
    process::Command,
};
use tar::Builder;

pub fn pack_layer() -> Result<()> {
    create_dir("/tmp/out")?;
    let status = Command::new("podman")
        .args([
            "run",
            "--rm",
            "-v",
            "/tmp/bundle:/bundle",
            "-v",
            "/tmp/out:/out",
            "registry.opensuse.org/opensuse/tumbleweed:latest",
            "sh",
            "-c",
            "cd /bundle && tar -czvf /out/bundle.tar.gz . && rm -rvf *",
        ])
        .status()?;
    if status.success() {
        copy("/tmp/out/bundle.tar.gz", "./bundle.tar.gz")?;
        remove_file("/tmp/out/bundle.tar.gz")?;
        Ok(())
    } else {
        Err(anyhow!("init of workspace failed"))
    }
}

pub fn pack_bundle() -> Result<()> {
    let tar_file = File::create("bundle.tar")?;
    let mut tar = Builder::new(tar_file);

    tar.append_dir_all(".", "/tmp/out")?;

    tar.finish()?;

    Ok(())
}
