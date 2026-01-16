//! API client types for the Couic firewall.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::ClientInput;
use crate::{ClientName, CompositeError, ErrorCode, Group, ValidateFrom};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRaw {
    pub name: ClientName,
    pub group: Group,
}

/// Client data stored in TOML files (name derived from filename).
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientFile {
    pub token: Uuid,
    pub group: Group,
}

/// Response structure for an API client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub name: ClientName,
    pub token: Uuid,
    pub group: Group,
}

impl TryFrom<ClientInput> for Client {
    type Error = CompositeError;

    fn try_from(input: ClientInput) -> Result<Self, Self::Error> {
        let mut errors = CompositeError::new(ErrorCode::Einvalid, "Validation failed");

        // Validate name
        let name = match ClientName::try_from(input.name) {
            Ok(n) => Some(n),
            Err(e) => {
                errors.add_detail("name", ErrorCode::Einvalid, &e.to_string());
                None
            }
        };

        // Validate group
        let group = match input.group.parse::<Group>() {
            Ok(g) => Some(g),
            Err(e) => {
                errors.add_detail("group", ErrorCode::Einvalid, &e.to_string());
                None
            }
        };

        if errors.has_errors() {
            return Err(errors);
        }

        let Some(name) = name else {
            return Err(errors);
        };

        let Some(group) = group else {
            return Err(errors);
        };

        // Both validated successfully - safe to unwrap
        Ok(Self {
            name,
            group,
            token: Uuid::new_v4(),
        })
    }
}

impl ValidateFrom for Client {
    type Input = ClientInput;

    fn validate_from(input: ClientInput) -> Result<Self, CompositeError> {
        Self::try_from(input)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_client_from_valid_input() {
        let input = ClientInput {
            name: "valid-client".to_string(),
            group: "admin".to_string(),
        };
        let client = Client::try_from(input);
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.name.as_str(), "valid-client");
        assert_eq!(client.group, Group::Admin);
    }

    #[test]
    fn test_client_from_invalid_name() {
        let input = ClientInput {
            name: String::new(),
            group: "admin".to_string(),
        };
        let result = Client::try_from(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_client_from_invalid_group() {
        let input = ClientInput {
            name: "valid-client".to_string(),
            group: "invalid-group".to_string(),
        };
        let result = Client::try_from(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_client_from_both_invalid() {
        let input = ClientInput {
            name: String::new(),
            group: "invalid".to_string(),
        };
        let result = Client::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.has_errors());
    }
}
