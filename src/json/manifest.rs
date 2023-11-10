use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ManifestOCI {
    Config: String,
    RepoTags: Vec<String>,
    Layers: Vec<String>,
}

impl ManifestOCI {
    pub fn new(config: String, image: String, layers: String) -> Self {
        Self {
            Config: config,
            RepoTags: vec![image],
            Layers: vec![layers],
        }
    }
}
