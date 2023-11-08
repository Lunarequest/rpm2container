use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Config {
    root: RootFs,
    process: Process,
}

#[derive(Debug, Serialize)]
pub struct RootFs {
    path: String,
    readonly: bool,
}

#[derive(Debug, Serialize)]
pub struct Process {
    terminal: bool,
    cwd: String,
    env: Option<Vec<String>>,
    args: Option<Vec<String>>,
    user: User,
}

#[derive(Debug, Serialize)]
pub struct User {
    uid: u32,
    gid: u32,
}

impl Config {
    pub fn new(env: Vec<String>, args: Vec<String>) -> Self {
        let root = RootFs {
            path: "rootfs".to_string(),
            readonly: false,
        };
        let user = User { uid: 0, gid: 0 };
        let process = Process {
            terminal: true,
            cwd: "/root".to_string(),
            env: Some(env),
            args: Some(args),
            user,
        };
        Config { root, process }
    }
}
