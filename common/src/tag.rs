use std::fmt;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::{MAX_TAG_NAME_LENGTH, RESERVED_TAG_NAME, SET_EXTENSION};

#[derive(Debug, Clone)]
pub struct InvalidTag(pub String);

impl fmt::Display for InvalidTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidTag {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(String);

impl Tag {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn is_set_tag(&self) -> bool {
        self.0.ends_with(SET_EXTENSION)
    }
}

impl TryFrom<String> for Tag {
    type Error = InvalidTag;

    fn try_from(tag: String) -> Result<Self, Self::Error> {
        if tag.len() > MAX_TAG_NAME_LENGTH {
            return Err(InvalidTag(format!(
                "Tag must be at most {MAX_TAG_NAME_LENGTH} characters"
            )));
        }

        if tag.ends_with(SET_EXTENSION) {
            return Err(InvalidTag(format!(
                "Tag cannot end with special suffix: {SET_EXTENSION}"
            )));
        }

        if tag.eq_ignore_ascii_case(RESERVED_TAG_NAME) {
            return Err(InvalidTag(format!(
                "Tag cannot be '{RESERVED_TAG_NAME}' (reserved for display)"
            )));
        }

        if !tag.is_empty()
            && !tag
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(InvalidTag(
                "Tag can only contain alphanumeric characters, hyphens, and underscores"
                    .to_string(),
            ));
        }

        Ok(Self(tag))
    }
}

impl TryFrom<&str> for Tag {
    type Error = InvalidTag;

    fn try_from(tag: &str) -> Result<Self, Self::Error> {
        Self::try_from(tag.to_string())
    }
}

impl FromStr for Tag {
    type Err = InvalidTag;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Tag> for String {
    fn from(tag: Tag) -> Self {
        tag.0
    }
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TagVisitor;

        impl Visitor<'_> for TagVisitor {
            type Value = Tag;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid tag string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Tag, E>
            where
                E: de::Error,
            {
                Tag::try_from(value).map_err(|e| de::Error::custom(e.0))
            }
        }

        deserializer.deserialize_str(TagVisitor)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_valid() {
        let tag = Tag::try_from("valid-tag_123").unwrap();
        assert_eq!(tag.as_str(), "valid-tag_123");
    }

    #[test]
    fn test_tag_empty() {
        let tag = Tag::try_from("").unwrap();
        assert_eq!(tag.as_str(), "");
    }

    #[test]
    fn test_tag_too_long() {
        let long_tag = "a".repeat(MAX_TAG_NAME_LENGTH + 1);
        let result = Tag::try_from(long_tag);
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("64"));
    }

    #[test]
    fn test_tag_forbidden_suffix() {
        let result = Tag::try_from("forbidden.couic");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains(".couic"));
    }

    #[test]
    fn test_tag_reserved_name() {
        let result = Tag::try_from("untagged");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("untagged"));

        // Case insensitive
        let result = Tag::try_from("UNTAGGED");
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_invalid_characters() {
        let result = Tag::try_from("invalid@tag#");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("alphanumeric"));
    }

    #[test]
    fn test_tag_serde_roundtrip() {
        let tag = Tag::try_from("serde-test").unwrap();
        let json = serde_json::to_string(&tag).unwrap();
        assert_eq!(json, "\"serde-test\"");
        let deserialized: Tag = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, tag);
    }

    #[test]
    fn test_tag_serde_invalid() {
        let result: Result<Tag, _> = serde_json::from_str("\"invalid@tag\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_is_set_tag() {
        let regular = Tag::try_from("regular").unwrap();
        assert!(!regular.is_set_tag());
    }
}
