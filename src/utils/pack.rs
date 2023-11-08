use anyhow::{anyhow, Result};
use std::{
    fs::{copy, create_dir},
    process::Command,
};

pub fn pack() -> Result<()> {
    create_dir("/tmp/out")?;
    let status = Command::new("podman")
        .args([
            "run",
            "--rm",
            "-v",
            format!("/tmp/bundle:/bundle").as_str(),
            "-v",
            format!("/tmp/out:/out").as_str(),
            "registry.opensuse.org/opensuse/tumbleweed:latest",
            "sh",
            "-c",
            "cd /bundle && tar -cvf /out/bundle.tar .",
        ])
        .status()?;
    if status.success() {
        copy("/tmp/out/bundle.tar", "./bundle.tar")?;
        Ok(())
    } else {
        return Err(anyhow!("init of workspace failed"));
    }
}
