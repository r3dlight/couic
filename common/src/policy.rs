use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{CompositeError, ErrorCode, ValidateFrom};

#[derive(Debug, Clone)]
pub struct InvalidPolicy(pub String);

impl fmt::Display for InvalidPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidPolicy {}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Policy {
    Drop,
    Ignore,
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drop => write!(f, "drop"),
            Self::Ignore => write!(f, "ignore"),
        }
    }
}

impl FromStr for Policy {
    type Err = InvalidPolicy;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "drop" => Ok(Self::Drop),
            "ignore" => Ok(Self::Ignore),
            _ => Err(InvalidPolicy(format!(
                "invalid policy: '{s}' (expected 'drop' or 'ignore')"
            ))),
        }
    }
}

impl TryFrom<String> for Policy {
    type Error = InvalidPolicy;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl ValidateFrom for Policy {
    type Input = String;

    fn validate_from(input: Self::Input) -> Result<Self, CompositeError> {
        Self::try_from(input).map_err(|e| {
            let mut err = CompositeError::new(ErrorCode::Ebadrequest, "Bad request");
            err.add_detail("policy", ErrorCode::Einvalid, &e.to_string());
            err
        })
    }
}
