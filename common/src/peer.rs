use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::rawentry::RawEntry;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct PeerJob {
    pub action: Action,
    pub entry: RawEntry,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::cidr::NormalizedCidr;
    use crate::expiration::Expiration;
    use crate::tag::Tag;
    use std::str::FromStr;

    #[test]
    fn test_peerjob_serde_roundtrip() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        let tag = Tag::try_from("peer-tag").unwrap();
        let job = PeerJob {
            action: Action::Add,
            entry: RawEntry {
                cidr,
                tag: Some(tag.clone()),
                expiration: Expiration::never(),
                metadata: None,
            },
        };

        let json = serde_json::to_string(&job).unwrap();
        let deserialized: PeerJob = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.action, Action::Add);
        assert_eq!(deserialized.entry.cidr, cidr);
        assert_eq!(deserialized.entry.tag, Some(tag));
    }
}
