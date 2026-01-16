use std::time::Duration;

use ipnet::IpNet;
use reqwest::blocking::Client;
use serde::Deserialize;

const TIMEOUT: Duration = Duration::from_secs(30);
const USER_AGENT_VALUE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

#[derive(Debug, thiserror::Error)]
pub enum RipeError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Failed to parse CIDR: {0}")]
    Parse(String),
    #[error("Invalid ASN format: {0}")]
    InvalidAsn(String),
}

#[derive(Deserialize, Debug)]
struct RipeResponse {
    data: RipeData,
}

#[derive(Deserialize, Debug)]
struct RipeData {
    prefixes: Vec<Prefix>,
}

#[derive(Deserialize, Debug)]
struct Prefix {
    prefix: String,
}

/// Fetch announced prefixes for a given ASN from RIPE NCC `RIPEstat` API
///
/// Accepts ASN in format "200373" or "AS200373"
/// Returns a list of IP prefixes (both IPv4 and IPv6)
pub fn fetch_asn_prefixes(asn: &str) -> Result<Vec<IpNet>, RipeError> {
    // Strip "AS" prefix if present
    let asn_clean = asn.trim().trim_start_matches("AS").trim_start_matches("as");

    // Validate ASN is numeric
    if asn_clean.is_empty() || !asn_clean.chars().all(|c| c.is_ascii_digit()) {
        return Err(RipeError::InvalidAsn(format!(
            "ASN must be numeric (e.g., '200373' or 'AS200373'), got: '{asn}'"
        )));
    }

    let url =
        format!("https://stat.ripe.net/data/announced-prefixes/data.json?resource=AS{asn_clean}");

    let client = Client::builder()
        .timeout(TIMEOUT)
        .user_agent(USER_AGENT_VALUE)
        .build()?;

    let response: RipeResponse = client.get(&url).send()?.json()?;

    let mut cidrs = Vec::new();
    let mut errors = Vec::new();

    for prefix_obj in response.data.prefixes {
        match prefix_obj.prefix.parse::<IpNet>() {
            Ok(cidr) => cidrs.push(cidr),
            Err(e) => errors.push(format!("{}: {}", prefix_obj.prefix, e)),
        }
    }

    if !errors.is_empty() && cidrs.is_empty() {
        return Err(RipeError::Parse(format!(
            "Failed to parse all prefixes: {}",
            errors.join(", ")
        )));
    }

    Ok(cidrs)
}
