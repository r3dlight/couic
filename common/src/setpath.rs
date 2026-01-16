use crate::api::SetPathInput;
use crate::{CompositeError, ErrorCode, Policy, SetName, ValidateFrom};

#[derive(Debug)]
pub struct SetPath {
    pub policy: Policy,
    pub name: SetName,
}

impl TryFrom<SetPathInput> for SetPath {
    type Error = CompositeError;

    fn try_from(input: SetPathInput) -> Result<Self, Self::Error> {
        let mut errors = CompositeError::new(ErrorCode::Ebadrequest, "Bad request");

        // Validate policy
        let policy = match Policy::try_from(input.policy) {
            Ok(p) => Some(p),
            Err(e) => {
                errors.add_detail("policy", ErrorCode::Einvalid, &e.to_string());
                None
            }
        };

        // Validate name
        let name = match SetName::try_from(input.name) {
            Ok(n) => Some(n),
            Err(e) => {
                errors.add_detail("name", ErrorCode::Einvalid, &e.to_string());
                None
            }
        };

        // If any validation failed, return all errors
        if errors.has_errors() {
            return Err(errors);
        }

        match (policy, name) {
            (Some(policy), Some(name)) => Ok(Self { policy, name }),
            _ => Err(errors),
        }
    }
}

impl ValidateFrom for SetPath {
    type Input = SetPathInput;

    fn validate_from(input: SetPathInput) -> Result<Self, CompositeError> {
        Self::try_from(input)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::api::SetPathInput;

    #[test]
    fn test_setpath_valid_drop() {
        let input = SetPathInput {
            policy: "drop".to_string(),
            name: "valid-set".to_string(),
        };
        let result = SetPath::try_from(input);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path.policy, Policy::Drop);
        assert_eq!(path.name.as_str(), "valid-set");
    }

    #[test]
    fn test_setpath_valid_ignore() {
        let input = SetPathInput {
            policy: "ignore".to_string(),
            name: "valid-set".to_string(),
        };
        let result = SetPath::try_from(input);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path.policy, Policy::Ignore);
    }

    #[test]
    fn test_setpath_policy_case_insensitive() {
        let input = SetPathInput {
            policy: "DROP".to_string(),
            name: "valid-set".to_string(),
        };
        let result = SetPath::try_from(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().policy, Policy::Drop);
    }

    #[test]
    fn test_setpath_invalid_policy() {
        let input = SetPathInput {
            policy: "invalid".to_string(),
            name: "valid-set".to_string(),
        };
        let result = SetPath::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("policy"));
    }

    #[test]
    fn test_setpath_invalid_name() {
        let input = SetPathInput {
            policy: "drop".to_string(),
            name: String::new(),
        };
        let result = SetPath::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("name"));
    }

    #[test]
    fn test_setpath_both_invalid() {
        let input = SetPathInput {
            policy: "invalid".to_string(),
            name: String::new(),
        };
        let result = SetPath::try_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("policy"));
        assert!(err.errors.contains_key("name"));
    }

    #[test]
    fn test_setpath_validate_from() {
        let input = SetPathInput {
            policy: "ignore".to_string(),
            name: "my-set".to_string(),
        };
        let result = SetPath::validate_from(input);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path.policy, Policy::Ignore);
        assert_eq!(path.name.as_str(), "my-set");
    }
}
