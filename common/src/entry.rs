use serde::{Deserialize, Serialize};

use crate::cidr::NormalizedCidr;
use crate::constants::SET_EXTENSION;
use crate::expiration::Expiration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Entry {
    pub creation: u64,
    pub cidr: NormalizedCidr,
    #[serde(default)]
    pub tag: Option<String>,
    pub expiration: Expiration,
}

impl Entry {
    #[must_use]
    pub fn in_set(&self) -> bool {
        self.tag
            .as_ref()
            .is_some_and(|tag| tag.ends_with(SET_EXTENSION))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_entry_in_set() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();

        // Entry with .couic extension should be in set
        let set_entry = Entry {
            creation: 0,
            cidr,
            tag: Some("testset.couic".to_string()),
            expiration: Expiration::never(),
        };
        assert!(set_entry.in_set());

        // Entry without .couic extension should not be in set
        let regular_entry = Entry {
            creation: 0,
            cidr,
            tag: Some("regular-tag".to_string()),
            expiration: Expiration::never(),
        };
        assert!(!regular_entry.in_set());

        // Entry without tag should not be in set
        let no_tag_entry = Entry {
            creation: 0,
            cidr,
            tag: None,
            expiration: Expiration::never(),
        };
        assert!(!no_tag_entry.in_set());
    }

    #[test]
    fn test_entry_serde_roundtrip() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        let entry = Entry {
            creation: 1000,
            cidr,
            tag: Some("serde-tag".to_string()),
            expiration: Expiration::from_timestamp(2000),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: Entry = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.creation, 1000);
        assert_eq!(deserialized.cidr, cidr);
        assert_eq!(deserialized.tag, Some("serde-tag".to_string()));
        assert_eq!(deserialized.expiration, Expiration::from_timestamp(2000));
    }
}
