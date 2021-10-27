use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::workers;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub db: DbConfig,
    #[serde(default)]
    pub workers: workers::Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub addr: SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub addr: String,
}

impl Config {
    pub fn load(path: PathBuf) -> eyre::Result<Self> {
        let raw_config = fs::read_to_string(path).map_err(Into::<eyre::Report>::into)?;
        serde_yaml::from_str(&raw_config).map_err(Into::<eyre::Report>::into)
    }
}
