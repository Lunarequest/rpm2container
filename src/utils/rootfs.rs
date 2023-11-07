use std::{fs::File, io::Write, path::PathBuf, process::Command};

use anyhow::{anyhow, Context, Result};
use tempfile::TempDir;

use crate::manifest::Manifest;

pub fn init_rootfs(root: &PathBuf, manifest: Manifest) -> Result<PathBuf> {
    let mut repo_commands = String::new();
    let initdir = TempDir::with_prefix("rpm2container-")?;
    let initfile_path = initdir.path().join("init.sh");
    for repo in manifest.contents.repositories {
        let ar = format!("zypper  --root /newroot ar -G -f {}\n", repo);
        repo_commands = repo_commands + &ar;
    }

    if repo_commands.is_empty() {
        return Err(anyhow!(
            "No repos defined, zypper will not be able to install anything"
        ));
    }

    let commands = format!(
        "
        #!/bin/bash -x
        {repo_commands}
        zypper --root /newroot in --no-recommends -y {}
        ",
        manifest.contents.packages.concat().to_string()
    );
    let mut init = File::create(&initfile_path)?;
    init.write_all(commands.as_bytes())?;

    let path = root.to_str().context("path was not kosher")?;
    let init_file = initfile_path.to_str().context("path was not kosher")?;
    eprintln!("{init_file}");
    let status = Command::new("podman")
        .args([
            "run",
            "--rm",
            "--cap-add",
            "CAP_SYS_CHROOT",
            "-v",
            format!("{path}:/newroot").as_str(),
            "-v",
            format!("{init_file}:/init.sh").as_str(),
            "registry.opensuse.org/opensuse/tumbleweed:latest",
            "/bin/bash",
            "-x",
            "/init.sh",
        ])
        .status()?;
    if status.success() {
        Ok(root.to_owned())
    } else {
        return Err(anyhow!("init of workspace failed"));
    }
}
