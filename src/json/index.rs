use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Index {
    schemaVersion: u16,
    mediaType: String,
    manifests: Vec<Manifest>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Manifest {
    mediaType: String,
    size: u64,
    digest: String,
    platform: Platform,
}

#[derive(Debug, Serialize)]
pub struct Platform {
    architecture: String,
    os: String,
}

impl Index {
    pub fn new(manifest_size: u64, manifest_digest: &String) -> Self {
        Self {
            schemaVersion: 2,
            mediaType: "application/vnd.oci.image.index.v1+json".to_string(),
            manifests: vec![Manifest {
                mediaType: "application/vnd.oci.image.index.v1+json".to_string(),
                size: manifest_size,
                digest: format!("sha256:{manifest_digest}"),
                platform: Platform {
                    architecture: "amd64".to_string(),
                    os: "linux".to_string(),
                },
            }],
        }
    }
}
