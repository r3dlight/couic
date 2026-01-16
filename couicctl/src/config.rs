use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Local,
    Remote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_file: String,
    pub mode: Mode,
    #[serde(default)]
    pub socket: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub tls: Option<bool>,
    #[serde(default)]
    pub token: Option<Uuid>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(&path).map_err(ConfigError::Io)?;

        let config: Self = toml::from_str(&content).map_err(ConfigError::Parse)?;

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        match self.mode {
            Mode::Local => {
                if self.socket.is_none() {
                    return Err(ConfigError::Validation(
                        "socket is required for local mode".to_string(),
                    ));
                }
            }
            Mode::Remote => {
                if self.host.is_none() {
                    return Err(ConfigError::Validation(
                        "host is required for remote mode".to_string(),
                    ));
                }
                if self.port.is_none() {
                    return Err(ConfigError::Validation(
                        "port is required for remote mode".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}
