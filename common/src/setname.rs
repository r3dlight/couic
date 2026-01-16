use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::constants::MAX_SET_NAME_LENGTH;

#[derive(Debug, Clone)]
pub struct InvalidSetName(pub String);

impl fmt::Display for InvalidSetName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidSetName {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetName(String);

impl SetName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SetName {
    type Error = InvalidSetName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(InvalidSetName("Set name cannot be empty".to_string()));
        }
        if name.len() > MAX_SET_NAME_LENGTH {
            return Err(InvalidSetName(format!(
                "Set name must be at most {MAX_SET_NAME_LENGTH} characters"
            )));
        }
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(InvalidSetName(
                "Set name can only contain alphanumeric characters, hyphens, and underscores"
                    .to_string(),
            ));
        }
        Ok(Self(name))
    }
}

impl TryFrom<&str> for SetName {
    type Error = InvalidSetName;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        Self::try_from(name.to_string())
    }
}

impl FromStr for SetName {
    type Err = InvalidSetName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl fmt::Display for SetName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for SetName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<SetName> for String {
    fn from(name: SetName) -> Self {
        name.0
    }
}

impl Serialize for SetName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SetName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_setname_valid() {
        let name = SetName::try_from("valid-set_123").unwrap();
        assert_eq!(name.as_str(), "valid-set_123");
    }

    #[test]
    fn test_setname_empty() {
        let result = SetName::try_from("");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("empty"));
    }

    #[test]
    fn test_setname_too_long() {
        let long_name = "a".repeat(MAX_SET_NAME_LENGTH + 1);
        let result = SetName::try_from(long_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("48"));
    }

    #[test]
    fn test_setname_max_length() {
        let max_name = "a".repeat(MAX_SET_NAME_LENGTH);
        let result = SetName::try_from(max_name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_setname_invalid_characters() {
        let result = SetName::try_from("invalid.name");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("alphanumeric"));

        let result = SetName::try_from("invalid name");
        assert!(result.is_err());

        let result = SetName::try_from("invalid@name");
        assert!(result.is_err());
    }

    #[test]
    fn test_setname_valid_characters() {
        assert!(SetName::try_from("abc123").is_ok());
        assert!(SetName::try_from("with-hyphen").is_ok());
        assert!(SetName::try_from("with_underscore").is_ok());
        assert!(SetName::try_from("MixedCase123").is_ok());
    }

    #[test]
    fn test_setname_from_str() {
        let name: SetName = "test-set".parse().unwrap();
        assert_eq!(name.as_str(), "test-set");
    }

    #[test]
    fn test_setname_display() {
        let name = SetName::try_from("my-set").unwrap();
        assert_eq!(name.to_string(), "my-set");
    }

    #[test]
    fn test_setname_into_string() {
        let name = SetName::try_from("my-set").unwrap();
        let s: String = name.into();
        assert_eq!(s, "my-set");
    }

    #[test]
    fn test_setname_serde_roundtrip() {
        let name = SetName::try_from("test-set").unwrap();
        let json = serde_json::to_string(&name).unwrap();
        assert_eq!(json, "\"test-set\"");
        let deserialized: SetName = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, name);
    }
}
