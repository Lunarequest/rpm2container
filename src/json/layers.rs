use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Layer {
    architecture: String,
    author: String,
    created: String,
    history: Vec<History>,
    os: String,
    rootfs: RootFs,
    config: Config,
}

#[derive(Debug, Serialize)]
pub struct History {
    author: String,
    created: String,
    created_by: String,
    comment: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Config {
    Entrypoint: Vec<String>,
    Env: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RootFs {
    pub r#type: String,
    pub diff_ids: Vec<String>,
}

impl Layer {
    pub fn new(hash: &String, args: Vec<String>, env: Vec<String>) -> Self {
        Self {
            architecture: "amd64".to_string(),
            author: "github.com/lunarequest/rpm2container".to_string(),
            created: "1970-01-01T00:00:00Z".to_string(),
            history: vec![History {
                author: "rpm2container".to_string(),
                created: "1970-01-01T00:00:00Z".to_string(),
                created_by: "rpm2container".to_string(),
                comment: "this is a rpm2container single-layer image".to_string(),
            }],
            os: "linux".to_string(),
            rootfs: RootFs {
                r#type: "layers".to_string(),
                diff_ids: vec![format!("sha256:{hash}")],
            },
            config: Config {
                Entrypoint: args,
                Env: env,
            },
        }
    }
}
