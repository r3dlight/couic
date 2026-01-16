use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    Eprocessing,
    Eunauthorized,
    Enotfound,
    Econflict,
    Ebadrequest,
    Einvalid,
    Einternal,
    Enotimplemented,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eprocessing => write!(f, "processing"),
            Self::Eunauthorized => write!(f, "unauthorized"),
            Self::Enotfound => write!(f, "not_found"),
            Self::Econflict => write!(f, "conflict"),
            Self::Ebadrequest => write!(f, "bad_request"),
            Self::Einvalid => write!(f, "invalid"),
            Self::Einternal => write!(f, "internal"),
            Self::Enotimplemented => write!(f, "not_implemented"),
        }
    }
}

impl FromStr for ErrorCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "processing" => Ok(Self::Eprocessing),
            "unauthorized" => Ok(Self::Eunauthorized),
            "not_found" => Ok(Self::Enotfound),
            "conflict" => Ok(Self::Econflict),
            "bad_request" => Ok(Self::Ebadrequest),
            "invalid" => Ok(Self::Einvalid),
            "internal" => Ok(Self::Einternal),
            "not_implemented" => Ok(Self::Enotimplemented),
            _ => Err(()),
        }
    }
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ErrorCodeVisitor;

        impl Visitor<'_> for ErrorCodeVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid error code string")
            }

            fn visit_str<E>(self, value: &str) -> Result<ErrorCode, E>
            where
                E: de::Error,
            {
                match value {
                    "processing" => Ok(ErrorCode::Eprocessing),
                    "unauthorized" => Ok(ErrorCode::Eunauthorized),
                    "not_found" => Ok(ErrorCode::Enotfound),
                    "conflict" => Ok(ErrorCode::Econflict),
                    "bad_request" => Ok(ErrorCode::Ebadrequest),
                    "invalid" => Ok(ErrorCode::Einvalid),
                    "internal" => Ok(ErrorCode::Einternal),
                    "not_implemented" => Ok(ErrorCode::Enotimplemented),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &[
                            "processing",
                            "unauthorized",
                            "not_found",
                            "conflict",
                            "bad_request",
                            "invalid",
                            "internal",
                            "not_implemented",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_str(ErrorCodeVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: ErrorCode,
    pub message: String,
}

impl fmt::Display for ErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code: {}, Message: {}", self.code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeError {
    pub code: ErrorCode,
    pub message: String,
    pub errors: HashMap<String, ErrorDetail>,
}

impl fmt::Display for CompositeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CompositeError {
    #[must_use]
    pub fn new(code: ErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            errors: HashMap::new(),
        }
    }

    pub fn add_detail(&mut self, field: &str, code: ErrorCode, message: &str) {
        self.errors.insert(
            field.to_string(),
            ErrorDetail {
                code,
                message: message.to_string(),
            },
        );
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::indexing_slicing)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_display() {
        assert_eq!(ErrorCode::Eprocessing.to_string(), "processing");
        assert_eq!(ErrorCode::Eunauthorized.to_string(), "unauthorized");
        assert_eq!(ErrorCode::Enotfound.to_string(), "not_found");
        assert_eq!(ErrorCode::Econflict.to_string(), "conflict");
        assert_eq!(ErrorCode::Ebadrequest.to_string(), "bad_request");
        assert_eq!(ErrorCode::Einvalid.to_string(), "invalid");
        assert_eq!(ErrorCode::Einternal.to_string(), "internal");
        assert_eq!(ErrorCode::Enotimplemented.to_string(), "not_implemented");
    }

    #[test]
    fn test_error_code_from_str() {
        assert_eq!(
            ErrorCode::from_str("processing"),
            Ok(ErrorCode::Eprocessing)
        );
        assert_eq!(
            ErrorCode::from_str("unauthorized"),
            Ok(ErrorCode::Eunauthorized)
        );
        assert_eq!(ErrorCode::from_str("not_found"), Ok(ErrorCode::Enotfound));
        assert_eq!(ErrorCode::from_str("conflict"), Ok(ErrorCode::Econflict));
        assert_eq!(
            ErrorCode::from_str("bad_request"),
            Ok(ErrorCode::Ebadrequest)
        );
        assert_eq!(ErrorCode::from_str("invalid"), Ok(ErrorCode::Einvalid));
        assert_eq!(ErrorCode::from_str("internal"), Ok(ErrorCode::Einternal));
        assert_eq!(
            ErrorCode::from_str("not_implemented"),
            Ok(ErrorCode::Enotimplemented)
        );
        assert_eq!(ErrorCode::from_str("unknown"), Err(()));
    }

    #[test]
    fn test_error_code_serde_roundtrip() {
        let code = ErrorCode::Einvalid;
        let json = serde_json::to_string(&code).unwrap();
        assert_eq!(json, "\"invalid\"");
        let deserialized: ErrorCode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ErrorCode::Einvalid);
    }

    #[test]
    fn test_composite_error_creation() {
        let error = CompositeError::new(ErrorCode::Einvalid, "Test error");
        assert_eq!(error.code, ErrorCode::Einvalid);
        assert_eq!(error.message, "Test error");
        assert!(error.errors.is_empty());
        assert!(!error.has_errors());
    }

    #[test]
    fn test_composite_error_add_detail() {
        let mut error = CompositeError::new(ErrorCode::Einvalid, "Validation failed");
        error.add_detail("field1", ErrorCode::Einvalid, "Field1 is invalid");
        error.add_detail("field2", ErrorCode::Enotfound, "Field2 not found");

        assert!(error.has_errors());
        assert_eq!(error.errors.len(), 2);
        assert_eq!(error.errors["field1"].message, "Field1 is invalid");
        assert_eq!(error.errors["field2"].code, ErrorCode::Enotfound);
    }

    #[test]
    fn test_composite_error_display() {
        let error = CompositeError::new(ErrorCode::Einvalid, "Something went wrong");
        assert_eq!(error.to_string(), "Something went wrong");
    }

    #[test]
    fn test_error_detail_display() {
        let detail = ErrorDetail {
            code: ErrorCode::Einvalid,
            message: "Invalid value".to_string(),
        };
        assert_eq!(detail.to_string(), "Code: invalid, Message: Invalid value");
    }

    #[test]
    fn test_composite_error_serde_roundtrip() {
        let mut error = CompositeError::new(ErrorCode::Einvalid, "Validation failed");
        error.add_detail("field1", ErrorCode::Einvalid, "Field1 is invalid");

        let json = serde_json::to_string(&error).unwrap();
        let deserialized: CompositeError = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.code, ErrorCode::Einvalid);
        assert_eq!(deserialized.message, "Validation failed");
        assert_eq!(deserialized.errors.len(), 1);
        assert_eq!(deserialized.errors["field1"].message, "Field1 is invalid");
    }
}
