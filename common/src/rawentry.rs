use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::expiration::deserialize_future_expiration;
use crate::{
    CompositeError, Entry, ErrorCode, Expiration, Metadata, NormalizedCidr, RawEntryInput, Tag,
    ValidateFrom,
};

/// A raw entry request (before processing).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RawEntry {
    pub cidr: NormalizedCidr,
    #[serde(default)]
    pub tag: Option<Tag>,
    #[serde(deserialize_with = "deserialize_future_expiration")]
    pub expiration: Expiration,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

impl RawEntry {
    #[must_use]
    pub fn into_entry(self) -> Entry {
        self.into_entry_and_metadata().0
    }

    pub fn into_entry_and_metadata(self) -> (Entry, Option<Metadata>) {
        let Self {
            cidr,
            tag,
            expiration,
            metadata,
        } = self;
        let entry = Entry {
            creation: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            cidr,
            tag: tag.map(String::from),
            expiration,
        };
        (entry, metadata)
    }
}

impl TryFrom<RawEntryInput> for RawEntry {
    type Error = CompositeError;

    fn try_from(input: RawEntryInput) -> Result<Self, Self::Error> {
        let mut errors = CompositeError::new(ErrorCode::Einvalid, "Validation failed");

        // Validate CIDR
        let cidr = match input.cidr.parse::<NormalizedCidr>() {
            Ok(c) => Some(c),
            Err(e) => {
                errors.add_detail("cidr", ErrorCode::Einvalid, &e.to_string());
                None
            }
        };

        // Validate Tag (if present and non-empty)
        let tag = match &input.tag {
            Some(t) if !t.is_empty() => match Tag::try_from(t.as_str()) {
                Ok(tag) => Some(tag),
                Err(e) => {
                    errors.add_detail("tag", ErrorCode::Einvalid, &e.0);
                    None
                }
            },
            _ => None,
        };

        // Validate Expiration
        let expiration = Expiration::from_timestamp(input.expiration);
        if !expiration.is_never() && expiration.is_expired() {
            errors.add_detail(
                "expiration",
                ErrorCode::Einvalid,
                "expiration timestamp must be in the future or zero",
            );
        }

        if errors.has_errors() {
            return Err(errors);
        }

        let Some(cidr) = cidr else {
            return Err(errors);
        };

        Ok(Self {
            cidr,
            tag,
            expiration,
            metadata: input.metadata,
        })
    }
}

impl ValidateFrom for RawEntry {
    type Input = RawEntryInput;

    fn validate_from(input: RawEntryInput) -> Result<Self, CompositeError> {
        Self::try_from(input)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::indexing_slicing)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_rawentry_into_entry() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        let raw = RawEntry {
            cidr,
            tag: Some(Tag::try_from("test-tag").unwrap()),
            expiration: Expiration::from_timestamp(12345),
            metadata: None,
        };

        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let entry = raw.into_entry();
        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert_eq!(entry.cidr, cidr);
        assert_eq!(entry.tag, Some("test-tag".to_string()));
        assert_eq!(entry.expiration, Expiration::from_timestamp(12345));
        assert!(entry.creation >= before && entry.creation <= after);
    }

    #[test]
    fn test_rawentry_into_entry_and_metadata() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        let metadata = Metadata {
            kind: "test-kind".to_string(),
            detail: "test-detail".to_string(),
            extra: None,
        };
        let raw = RawEntry {
            cidr,
            tag: None,
            expiration: Expiration::never(),
            metadata: Some(metadata),
        };

        let (entry, extracted_metadata) = raw.into_entry_and_metadata();
        assert_eq!(entry.cidr, cidr);
        assert!(extracted_metadata.is_some());
        let meta = extracted_metadata.unwrap();
        assert_eq!(meta.kind, "test-kind");
        assert_eq!(meta.detail, "test-detail");
    }

    #[test]
    fn test_valid_raw_entry_input() {
        let input = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: Some("valid-tag".to_string()),
            expiration: 4_102_444_800, // Year 2100
            metadata: None,
        };

        let result = RawEntry::try_from(input);
        assert!(result.is_ok());
        let entry = result.unwrap();
        assert_eq!(entry.cidr.to_string(), "192.168.1.0/24");
        assert_eq!(entry.tag.as_ref().map(Tag::as_str), Some("valid-tag"));
    }

    #[test]
    fn test_invalid_cidr_only() {
        let input = RawEntryInput {
            cidr: "not-a-cidr".to_string(),
            tag: Some("valid-tag".to_string()),
            expiration: 4_102_444_800,
            metadata: None,
        };

        let result = RawEntry::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 1);
        assert!(err.errors.contains_key("cidr"));
    }

    #[test]
    fn test_multiple_validation_errors() {
        let input = RawEntryInput {
            cidr: "not-a-cidr".to_string(),
            tag: Some("invalid@tag".to_string()),
            expiration: 1000, // Past timestamp
            metadata: None,
        };

        let result = RawEntry::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();

        // Should have errors for cidr, tag, and expiration
        assert_eq!(err.errors.len(), 3);
        assert!(err.errors.contains_key("cidr"));
        assert!(err.errors.contains_key("tag"));
        assert!(err.errors.contains_key("expiration"));
    }

    #[test]
    fn test_expiration_zero_is_valid() {
        let input = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: None,
            expiration: 0, // Never expires
            metadata: None,
        };

        let result = RawEntry::try_from(input);
        assert!(result.is_ok());
        assert!(result.unwrap().expiration.is_never());
    }

    #[test]
    fn test_empty_tag_is_valid() {
        let input = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: Some(String::new()),
            expiration: 4_102_444_800,
            metadata: None,
        };

        let result = RawEntry::try_from(input);
        assert!(result.is_ok());
        // Empty tag becomes None
        assert!(result.unwrap().tag.is_none());
    }

    #[test]
    fn test_tag_too_long() {
        let input = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: Some("a".repeat(65)),
            expiration: 4_102_444_800,
            metadata: None,
        };

        let err = RawEntry::try_from(input).unwrap_err();
        assert!(err.errors.contains_key("tag"));
        assert!(err.errors["tag"].message.contains("64"));
    }

    #[test]
    fn test_tag_reserved_name() {
        let input = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: Some("untagged".to_string()),
            expiration: 4_102_444_800,
            metadata: None,
        };

        let err = RawEntry::try_from(input).unwrap_err();
        assert!(err.errors.contains_key("tag"));
        assert!(err.errors["tag"].message.contains("untagged"));
    }

    #[test]
    fn test_tag_forbidden_suffix() {
        let input = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: Some("test.couic".to_string()),
            expiration: 4_102_444_800,
            metadata: None,
        };

        let err = RawEntry::try_from(input).unwrap_err();
        assert!(err.errors.contains_key("tag"));
        assert!(err.errors["tag"].message.contains(".couic"));
    }

    #[test]
    fn test_validate_from_valid() {
        let dto = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: Some("valid-tag_123".to_string()),
            expiration: 4_102_444_800, // Year 2100
            metadata: None,
        };

        let result = RawEntry::validate_from(dto);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_from_invalid_cidr() {
        let dto = RawEntryInput {
            cidr: "not-a-cidr".to_string(),
            tag: None,
            expiration: 4_102_444_800,
            metadata: None,
        };

        let result = RawEntry::validate_from(dto);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("cidr"));
    }

    #[test]
    fn test_validate_from_past_expiration() {
        let dto = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: None,
            expiration: 1000, // 1970, definitely past
            metadata: None,
        };

        let result = RawEntry::validate_from(dto);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("expiration"));
    }

    #[test]
    fn test_validate_from_future_expiration() {
        let dto = RawEntryInput {
            cidr: "192.168.1.0/24".to_string(),
            tag: None,
            expiration: 4_102_444_800, // Year 2100
            metadata: None,
        };

        let result = RawEntry::validate_from(dto);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_from_multiple_errors() {
        let dto = RawEntryInput {
            cidr: "not-a-cidr".to_string(),
            tag: Some("invalid@tag".to_string()),
            expiration: 1000, // Past expiration
            metadata: None,
        };

        let result = RawEntry::validate_from(dto);
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Should have errors for cidr, tag, and expiration
        assert_eq!(err.errors.len(), 3);
        assert!(err.errors.contains_key("cidr"));
        assert!(err.errors.contains_key("tag"));
        assert!(err.errors.contains_key("expiration"));
    }
}
