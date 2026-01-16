use ipnet::IpNet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper around `IpNet` that guarantees the CIDR is normalized to its network address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NormalizedCidr(IpNet);

impl NormalizedCidr {
    #[must_use]
    pub fn new(cidr: IpNet) -> Self {
        Self(cidr.trunc())
    }

    pub fn from_addr_and_prefix(
        addr: std::net::IpAddr,
        prefix_len: u8,
    ) -> Result<Self, ipnet::PrefixLenError> {
        let ipnet = match addr {
            std::net::IpAddr::V4(ipv4) => IpNet::V4(ipnet::Ipv4Net::new(ipv4, prefix_len)?),
            std::net::IpAddr::V6(ipv6) => IpNet::V6(ipnet::Ipv6Net::new(ipv6, prefix_len)?),
        };
        Ok(Self(ipnet.trunc()))
    }

    #[must_use]
    pub const fn inner(&self) -> IpNet {
        self.0
    }

    #[must_use]
    pub fn network(&self) -> std::net::IpAddr {
        self.0.network()
    }

    #[must_use]
    pub fn prefix_len(&self) -> u8 {
        self.0.prefix_len()
    }

    #[must_use]
    pub const fn is_v4(&self) -> bool {
        matches!(self.0, IpNet::V4(_))
    }

    #[must_use]
    pub const fn is_v6(&self) -> bool {
        matches!(self.0, IpNet::V6(_))
    }

    #[must_use]
    pub fn to_lpm_key_v4(self) -> Option<(u32, u32)> {
        match self.0 {
            IpNet::V4(net) => Some((
                u32::from(net.prefix_len()),
                u32::from(net.network()).to_be(),
            )),
            IpNet::V6(_) => None,
        }
    }

    #[must_use]
    pub fn to_lpm_key_v6(self) -> Option<(u32, u128)> {
        match self.0 {
            IpNet::V4(_) => None,
            IpNet::V6(net) => Some((
                u32::from(net.prefix_len()),
                u128::from(net.network()).to_be(),
            )),
        }
    }
}

impl From<IpNet> for NormalizedCidr {
    fn from(cidr: IpNet) -> Self {
        Self::new(cidr)
    }
}

impl From<NormalizedCidr> for IpNet {
    fn from(normalized: NormalizedCidr) -> Self {
        normalized.0
    }
}

impl std::fmt::Display for NormalizedCidr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for NormalizedCidr {
    type Err = ipnet::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cidr: IpNet = s.parse()?;
        Ok(Self::new(cidr))
    }
}

impl Serialize for NormalizedCidr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NormalizedCidr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cidr = IpNet::deserialize(deserializer)?;
        Ok(Self::new(cidr))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_normalized_cidr_from_addr_and_prefix_ipv4() {
        use std::net::Ipv4Addr;
        let addr = std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 45));
        let normalized = NormalizedCidr::from_addr_and_prefix(addr, 24).unwrap();

        // Should be normalized to network address
        assert_eq!(normalized.to_string(), "192.168.1.0/24");
        assert!(normalized.is_v4());
    }

    #[test]
    fn test_normalized_cidr_from_addr_and_prefix_ipv6() {
        use std::net::Ipv6Addr;
        let addr = std::net::IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
        let normalized = NormalizedCidr::from_addr_and_prefix(addr, 64).unwrap();

        assert_eq!(normalized.to_string(), "2001:db8::/64");
        assert!(normalized.is_v6());
    }

    #[test]
    fn test_normalized_cidr_invalid_prefix_length() {
        use std::net::Ipv4Addr;
        let addr = std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

        // IPv4 prefix must be 0-32
        let result = NormalizedCidr::from_addr_and_prefix(addr, 33);
        assert!(result.is_err());
    }

    #[test]
    fn test_normalized_cidr_new_normalizes() {
        // 192.168.1.100/24 should normalize to 192.168.1.0/24
        let non_normalized: IpNet = "192.168.1.100/24".parse().unwrap();
        let normalized = NormalizedCidr::new(non_normalized);
        assert_eq!(normalized.to_string(), "192.168.1.0/24");
    }

    #[test]
    fn test_normalized_cidr_inner() {
        let cidr = NormalizedCidr::from_str("10.0.0.0/8").unwrap();
        let inner: IpNet = cidr.inner();
        assert_eq!(inner.to_string(), "10.0.0.0/8");
    }

    #[test]
    fn test_normalized_cidr_network() {
        let cidr = NormalizedCidr::from_str("172.16.5.0/16").unwrap();
        assert_eq!(cidr.network().to_string(), "172.16.0.0");
    }

    #[test]
    fn test_normalized_cidr_to_lpm_key_v4() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        let (prefix_len, addr_be) = cidr.to_lpm_key_v4().unwrap();
        assert_eq!(prefix_len, 24);
        // 192.168.1.0 in big-endian
        assert_eq!(addr_be, 0xc0a8_0100_u32.to_be());
    }

    #[test]
    fn test_normalized_cidr_to_lpm_key_v4_returns_none_for_v6() {
        let cidr = NormalizedCidr::from_str("2001:db8::/32").unwrap();
        assert!(cidr.to_lpm_key_v4().is_none());
    }

    #[test]
    fn test_normalized_cidr_to_lpm_key_v6() {
        let cidr = NormalizedCidr::from_str("2001:db8::/32").unwrap();
        let (prefix_len, addr_be) = cidr.to_lpm_key_v6().unwrap();
        assert_eq!(prefix_len, 32);
        // 2001:db8:: in big-endian
        let expected: u128 = 0x2001_0db8_0000_0000_0000_0000_0000_0000;
        assert_eq!(addr_be, expected.to_be());
    }

    #[test]
    fn test_normalized_cidr_to_lpm_key_v6_returns_none_for_v4() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        assert!(cidr.to_lpm_key_v6().is_none());
    }

    #[test]
    fn test_normalized_cidr_from_into_ipnet() {
        let original: IpNet = "10.20.30.0/24".parse().unwrap();
        let normalized: NormalizedCidr = original.into();
        let back: IpNet = normalized.into();
        assert_eq!(back.to_string(), "10.20.30.0/24");
    }

    #[test]
    fn test_normalized_cidr_serde_roundtrip() {
        let cidr = NormalizedCidr::from_str("192.168.1.0/24").unwrap();
        let json = serde_json::to_string(&cidr).unwrap();
        assert_eq!(json, "\"192.168.1.0/24\"");
        let deserialized: NormalizedCidr = serde_json::from_str(&json).unwrap();
        assert_eq!(cidr, deserialized);
    }
}
