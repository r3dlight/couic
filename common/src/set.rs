use std::fmt;

use ipnet::IpNet;
use serde::{Deserialize, Serialize};

use crate::api::SetInput;
use crate::constants::MAX_SET_FILE_SIZE;
use crate::error::{CompositeError, ErrorCode};
use crate::setname::SetName;
use crate::validation::ValidateFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    pub name: SetName,
    pub entries: Vec<IpNet>,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Set: {}", self.name)?;
        writeln!(f, "Entry count: {}", self.entries.len())?;
        writeln!(f, "Entries:")?;
        for entry in &self.entries {
            writeln!(f, "\t{entry}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSummary {
    pub name: SetName,
    pub entry_count: usize,
    pub file_size: u64,
}

impl fmt::Display for SetSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} entries ({} bytes)",
            self.name, self.entry_count, self.file_size
        )
    }
}

pub fn validate_set_entries_size(entries: &[IpNet], errors: &mut CompositeError) {
    let approx_size: usize = entries
        .iter()
        .map(|e| e.to_string().len().saturating_add(1))
        .sum();
    #[allow(clippy::integer_division)]
    if approx_size as u64 > MAX_SET_FILE_SIZE {
        errors.add_detail(
            "entries",
            ErrorCode::Einvalid,
            &format!(
                "Total entries would exceed {} MB limit",
                MAX_SET_FILE_SIZE / 1024 / 1024
            ),
        );
    }
}

impl ValidateFrom for Set {
    type Input = SetInput;

    fn validate_from(input: SetInput) -> Result<Self, CompositeError> {
        let mut errors = CompositeError::new(ErrorCode::Einvalid, "Validation failed");

        // Validate name
        let name = match SetName::try_from(input.name) {
            Ok(n) => Some(n),
            Err(e) => {
                errors.add_detail("name", ErrorCode::Einvalid, &e.0);
                None
            }
        };

        // Validate entries (parse each CIDR string)
        let mut entries = Vec::with_capacity(input.entries.len());
        for (i, entry_str) in input.entries.iter().enumerate() {
            match entry_str.parse::<IpNet>() {
                Ok(cidr) => entries.push(cidr),
                Err(e) => {
                    errors.add_detail(
                        &format!("entries[{i}]"),
                        ErrorCode::Einvalid,
                        &e.to_string(),
                    );
                }
            }
        }

        // Validate total size if we have valid entries
        if !entries.is_empty() {
            validate_set_entries_size(&entries, &mut errors);
        }

        if errors.has_errors() {
            return Err(errors);
        }

        let Some(name) = name else {
            return Err(errors);
        };

        Ok(Self { name, entries })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::api::SetInput;

    #[test]
    fn test_set_valid() {
        let input = SetInput {
            name: "valid-set".to_string(),
            entries: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
        };
        let set = Set::validate_from(input);
        assert!(set.is_ok());
        let set = set.unwrap();
        assert_eq!(set.name.as_str(), "valid-set");
        assert_eq!(set.entries.len(), 2);
    }

    #[test]
    fn test_set_invalid_name() {
        let input = SetInput {
            name: String::new(),
            entries: vec!["192.168.1.0/24".to_string()],
        };
        let result = Set::validate_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("name"));
    }

    #[test]
    fn test_set_invalid_entry() {
        let input = SetInput {
            name: "valid-set".to_string(),
            entries: vec!["not-a-cidr".to_string()],
        };
        let result = Set::validate_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("entries[0]"));
    }

    #[test]
    fn test_set_multiple_invalid_entries() {
        let input = SetInput {
            name: "valid-set".to_string(),
            entries: vec![
                "192.168.1.0/24".to_string(),
                "invalid1".to_string(),
                "10.0.0.0/8".to_string(),
                "invalid2".to_string(),
            ],
        };
        let result = Set::validate_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("entries[1]"));
        assert!(err.errors.contains_key("entries[3]"));
        assert!(!err.errors.contains_key("entries[0]"));
        assert!(!err.errors.contains_key("entries[2]"));
    }

    #[test]
    fn test_set_empty_entries() {
        let input = SetInput {
            name: "empty-set".to_string(),
            entries: vec![],
        };
        let result = Set::validate_from(input);
        assert!(result.is_ok());
        let set = result.unwrap();
        assert!(set.entries.is_empty());
    }

    #[test]
    fn test_set_both_invalid() {
        let input = SetInput {
            name: String::new(),
            entries: vec!["not-a-cidr".to_string()],
        };
        let result = Set::validate_from(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.errors.contains_key("name"));
        assert!(err.errors.contains_key("entries[0]"));
    }

    #[test]
    fn test_set_display() {
        let input = SetInput {
            name: "test-set".to_string(),
            entries: vec!["192.168.1.0/24".to_string()],
        };
        let set = Set::validate_from(input).unwrap();
        let display = set.to_string();
        assert!(display.contains("Set: test-set"));
        assert!(display.contains("Entry count: 1"));
        assert!(display.contains("192.168.1.0/24"));
    }

    #[test]
    fn test_set_summary_display() {
        let summary = SetSummary {
            name: SetName::try_from("my-set").unwrap(),
            entry_count: 42,
            file_size: 1024,
        };
        let display = summary.to_string();
        assert!(display.contains("my-set"));
        assert!(display.contains("42 entries"));
        assert!(display.contains("1024 bytes"));
    }

    #[test]
    fn test_validate_set_entries_size_within_limit() {
        let entries: Vec<IpNet> = vec!["192.168.1.0/24".parse().unwrap()];
        let mut errors = CompositeError::new(ErrorCode::Einvalid, "Test");
        validate_set_entries_size(&entries, &mut errors);
        assert!(!errors.has_errors());
    }

    #[test]
    fn test_set_ipv6_entries() {
        let input = SetInput {
            name: "ipv6-set".to_string(),
            entries: vec!["2001:db8::/32".to_string(), "::1/128".to_string()],
        };
        let result = Set::validate_from(input);
        assert!(result.is_ok());
        let set = result.unwrap();
        assert_eq!(set.entries.len(), 2);
    }
}
