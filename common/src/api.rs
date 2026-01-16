use serde::Deserialize;

use crate::Metadata;

#[derive(Debug, Clone, Deserialize)]
pub struct RawEntryInput {
    pub cidr: String,
    #[serde(default)]
    pub tag: Option<String>,
    pub expiration: u64,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetInput {
    pub name: String,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClientInput {
    pub name: String,
    pub group: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetPathInput {
    pub policy: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyPathInput {
    pub policy: String,
    pub ip: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PeerJobInput {
    pub action: String,
    pub entry: RawEntryInput,
}
