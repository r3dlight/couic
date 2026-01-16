use crate::api::PolicyPathInput;
use crate::{CompositeError, ErrorCode, NormalizedCidr, Policy, ValidateFrom};

pub struct PolicyPath {
    pub policy: Policy,
    pub cidr: NormalizedCidr,
}

impl TryFrom<PolicyPathInput> for PolicyPath {
    type Error = CompositeError;

    fn try_from(input: PolicyPathInput) -> Result<Self, Self::Error> {
        let mut errors = CompositeError::new(ErrorCode::Ebadrequest, "Bad request");

        let policy = match Policy::try_from(input.policy) {
            Ok(p) => Some(p),
            Err(e) => {
                errors.add_detail("policy", ErrorCode::Einvalid, &e.to_string());
                None
            }
        };

        let prefix = if let Ok(p) = input.prefix.parse::<u8>() {
            Some(p)
        } else {
            errors.add_detail(
                "prefix",
                ErrorCode::Einvalid,
                &format!("{} is not a valid prefix value", input.prefix),
            );
            None
        };

        let ip = if let Ok(ip) = input.ip.parse::<std::net::IpAddr>() {
            Some(ip)
        } else {
            errors.add_detail(
                "ip",
                ErrorCode::Einvalid,
                &format!("{} is not a valid IP address", input.ip),
            );
            None
        };

        // ---- CIDR validation (only if ip + prefix exist) ----
        let cidr = match (ip, prefix) {
            (Some(ip), Some(prefix)) => {
                let max = match ip {
                    std::net::IpAddr::V4(_) => 32,
                    std::net::IpAddr::V6(_) => 128,
                };

                if prefix > max {
                    errors.add_detail(
                        "prefix",
                        ErrorCode::Einvalid,
                        &format!("Prefix must be between 0 and {max}, got {prefix}"),
                    );
                    None
                } else {
                    match NormalizedCidr::from_addr_and_prefix(ip, prefix) {
                        Ok(cidr) => Some(cidr),
                        Err(e) => {
                            errors.add_detail("cidr", ErrorCode::Einvalid, &e.to_string());
                            None
                        }
                    }
                }
            }
            _ => None,
        };

        // If any validation failed, return all errors
        if errors.has_errors() {
            return Err(errors);
        }

        match (policy, cidr) {
            (Some(policy), Some(cidr)) => Ok(Self { policy, cidr }),
            _ => Err(errors),
        }
    }
}

impl ValidateFrom for PolicyPath {
    type Input = PolicyPathInput;

    fn validate_from(input: PolicyPathInput) -> Result<Self, CompositeError> {
        Self::try_from(input)
    }
}
