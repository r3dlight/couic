use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct InvalidGroup(pub String);

impl fmt::Display for InvalidGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidGroup {}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Group {
    Admin,
    ClientRo,
    ClientRw,
    Monitoring,
    Peering,
}

impl FromStr for Group {
    type Err = InvalidGroup;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Self::Admin),
            "clientro" => Ok(Self::ClientRo),
            "clientrw" => Ok(Self::ClientRw),
            "monitoring" => Ok(Self::Monitoring),
            "peering" => Ok(Self::Peering),
            _ => Err(InvalidGroup(format!("Invalid group: {s}"))),
        }
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Admin => "admin",
                Self::ClientRo => "client_ro",
                Self::ClientRw => "client_rw",
                Self::Monitoring => "monitoring",
                Self::Peering => "peering",
            }
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_group_from_str_admin() {
        let group: Group = "admin".parse().unwrap();
        assert_eq!(group, Group::Admin);
    }

    #[test]
    fn test_group_from_str_clientro() {
        let group: Group = "clientro".parse().unwrap();
        assert_eq!(group, Group::ClientRo);
    }

    #[test]
    fn test_group_from_str_clientrw() {
        let group: Group = "clientrw".parse().unwrap();
        assert_eq!(group, Group::ClientRw);
    }

    #[test]
    fn test_group_from_str_monitoring() {
        let group: Group = "monitoring".parse().unwrap();
        assert_eq!(group, Group::Monitoring);
    }

    #[test]
    fn test_group_from_str_peering() {
        let group: Group = "peering".parse().unwrap();
        assert_eq!(group, Group::Peering);
    }

    #[test]
    fn test_group_from_str_case_insensitive() {
        let group: Group = "ADMIN".parse().unwrap();
        assert_eq!(group, Group::Admin);

        let group: Group = "Admin".parse().unwrap();
        assert_eq!(group, Group::Admin);
    }

    #[test]
    fn test_group_from_str_invalid() {
        let result: Result<Group, _> = "invalid".parse();
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("Invalid group"));
    }

    #[test]
    fn test_group_display() {
        assert_eq!(Group::Admin.to_string(), "admin");
        assert_eq!(Group::ClientRo.to_string(), "client_ro");
        assert_eq!(Group::ClientRw.to_string(), "client_rw");
        assert_eq!(Group::Monitoring.to_string(), "monitoring");
        assert_eq!(Group::Peering.to_string(), "peering");
    }
}
