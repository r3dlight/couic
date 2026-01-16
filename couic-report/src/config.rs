use serde::Deserialize;
use std::fs;
use toml::from_str;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThresholdMin {
    #[default]
    Green,
    Orange,
    Red,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Thresholds {
    #[serde(default = "default_orange")]
    pub orange: usize,
    #[serde(default = "default_red")]
    pub red: usize,
    #[serde(default = "default_threshold_min")]
    pub threshold_min: ThresholdMin,
}

fn default_orange() -> usize {
    10
}
fn default_red() -> usize {
    100
}
fn default_threshold_min() -> ThresholdMin {
    ThresholdMin::Green
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            orange: default_orange(),
            red: default_red(),
            threshold_min: default_threshold_min(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub discord: Option<DiscordConfig>,
    #[serde(default = "default_batch_interval")]
    pub batch_interval_secs: u64,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default = "default_server")]
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    #[serde(default = "default_server_name")]
    pub name: String,
    #[serde(default = "default_server_addr")]
    pub addr: String,
    #[serde(default = "default_server_port")]
    pub port: u16,
    pub secret: Uuid,
}

fn default_batch_interval() -> u64 {
    900
}

fn default_server_name() -> String {
    "localhost".to_string()
}
fn default_server_addr() -> String {
    "127.0.0.1".to_string()
}
fn default_server_port() -> u16 {
    8000
}

fn default_server() -> Server {
    Server {
        name: default_server_name(),
        addr: default_server_addr(),
        port: default_server_port(),
        secret: Uuid::nil(),
    }
}

pub fn load_config(path: &str) -> Result<Config, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("failed to read config.toml: {e}"))?;
    let cfg: Config = from_str(&content).map_err(|e| format!("invalid config.toml: {e}"))?;
    if cfg.thresholds.red <= cfg.thresholds.orange {
        return Err(format!(
            "Invalid thresholds: red ({}) must be greater than orange ({})",
            cfg.thresholds.red, cfg.thresholds.orange
        ));
    }
    if cfg.server.secret.is_nil() {
        return Err("secret cannot be empty in [server] configuration".to_string());
    }
    // Only allow localhost addresses
    let allowed_addrs = ["localhost", "127.0.0.1", "::1"];
    if !allowed_addrs.contains(&cfg.server.addr.as_str()) {
        return Err(format!(
            "server.addr must be one of: localhost, 127.0.0.1, ::1 (got '{}')",
            cfg.server.addr
        ));
    }
    Ok(cfg)
}
