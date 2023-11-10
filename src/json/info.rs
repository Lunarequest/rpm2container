use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Info {
    schemaVersion: u16,
    mediaType: String,
    config: Config,
    layers: Vec<LayerInfo>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Config {
    mediaType: String,
    size: u64,
    digest: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct LayerInfo {
    mediaType: String,
    size: u64,
    digest: String,
}

impl Info {
    pub fn new(
        config_size: u64,
        config_digest: &String,
        layer_size: u64,
        lazyer_digest: &String,
    ) -> Self {
        Self {
            schemaVersion: 2,
            mediaType: "application/vnd.oci.image.manifest.v1+json".to_owned(),
            config: Config {
                mediaType: "application/vnd.oci.image.config.v1+json".to_string(),
                size: config_size,
                digest: config_digest.to_owned(),
            },
            layers: vec![LayerInfo {
                mediaType: "application/vnd.oci.image.layer.v1.tar+gzip".to_string(),
                size: layer_size,
                digest: lazyer_digest.to_owned(),
            }],
        }
    }
}
