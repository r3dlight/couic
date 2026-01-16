use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{CompositeError, ErrorCode, MAX_CLIENT_NAME_LENGTH, ValidateFrom};

#[derive(Debug, Clone)]
pub struct InvalidClientName(pub String);

impl fmt::Display for InvalidClientName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidClientName {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientName(String);

impl ClientName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ClientName {
    type Error = InvalidClientName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(InvalidClientName("Client name cannot be empty".to_string()));
        }
        if name.len() > MAX_CLIENT_NAME_LENGTH {
            return Err(InvalidClientName(format!(
                "Client name must be at most {MAX_CLIENT_NAME_LENGTH} characters"
            )));
        }
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(InvalidClientName(
                "Client name can only contain alphanumeric characters, hyphens, and underscores"
                    .to_string(),
            ));
        }
        Ok(Self(name))
    }
}

impl TryFrom<&str> for ClientName {
    type Error = InvalidClientName;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        Self::try_from(name.to_string())
    }
}

impl FromStr for ClientName {
    type Err = InvalidClientName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_string())
    }
}

impl fmt::Display for ClientName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ClientName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ClientName> for String {
    fn from(name: ClientName) -> Self {
        name.0
    }
}

impl Serialize for ClientName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ClientName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}

impl ValidateFrom for ClientName {
    type Input = String;

    fn validate_from(input: Self::Input) -> Result<Self, CompositeError> {
        Self::try_from(input).map_err(|e| {
            let mut err = CompositeError::new(ErrorCode::Ebadrequest, "Bad request");
            err.add_detail("name", ErrorCode::Einvalid, &e.to_string());
            err
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_clientname_valid() {
        let name = ClientName::try_from("valid-client_123").unwrap();
        assert_eq!(name.as_str(), "valid-client_123");
    }

    #[test]
    fn test_clientname_empty() {
        let result = ClientName::try_from("");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("empty"));
    }

    #[test]
    fn test_clientname_too_long() {
        let long_name = "a".repeat(MAX_CLIENT_NAME_LENGTH + 1);
        let result = ClientName::try_from(long_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("48"));
    }

    #[test]
    fn test_clientname_max_length() {
        let max_name = "a".repeat(MAX_CLIENT_NAME_LENGTH);
        let result = ClientName::try_from(max_name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clientname_invalid_characters() {
        let result = ClientName::try_from("invalid.name");
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("alphanumeric"));

        let result = ClientName::try_from("invalid name");
        assert!(result.is_err());

        let result = ClientName::try_from("invalid@name");
        assert!(result.is_err());
    }

    #[test]
    fn test_clientname_valid_characters() {
        assert!(ClientName::try_from("abc123").is_ok());
        assert!(ClientName::try_from("with-hyphen").is_ok());
        assert!(ClientName::try_from("with_underscore").is_ok());
        assert!(ClientName::try_from("MixedCase123").is_ok());
    }

    #[test]
    fn test_clientname_from_str() {
        let name: ClientName = "test-client".parse().unwrap();
        assert_eq!(name.as_str(), "test-client");
    }

    #[test]
    fn test_clientname_display() {
        let name = ClientName::try_from("my-client").unwrap();
        assert_eq!(name.to_string(), "my-client");
    }

    #[test]
    fn test_clientname_into_string() {
        let name = ClientName::try_from("my-client").unwrap();
        let s: String = name.into();
        assert_eq!(s, "my-client");
    }
}
