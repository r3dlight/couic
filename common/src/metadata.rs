use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub kind: String,
    pub detail: String,
    pub extra: Option<Map<String, Value>>,
}
