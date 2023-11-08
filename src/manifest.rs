use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub contents: Contents,
    pub entrypoint: Entrypoint,
}

#[derive(Deserialize)]
pub struct Contents {
    pub repositories: Vec<String>,
    pub packages: Vec<String>,
}

#[derive(Deserialize)]
pub struct Entrypoint {
    pub command: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
}
