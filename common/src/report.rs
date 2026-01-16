use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::entry::Entry;
use crate::metadata::Metadata;
use crate::policy::Policy;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Report {
    pub action: Action,
    pub policy: Policy,
    pub entry: Entry,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}
