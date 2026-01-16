use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
#[repr(C)]
pub struct PktStats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
}

unsafe impl aya::Pod for PktStats {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub drop_cidr_count: usize,
    pub ignore_cidr_count: usize,
    pub xdp: HashMap<String, PktStats>,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Drop CIDR Count: {}\nIgnore CIDR Count: {}\nXDP Stats:\n",
            self.drop_cidr_count, self.ignore_cidr_count
        )?;
        let mut actions: Vec<_> = self.xdp.keys().collect();
        actions.sort();
        for action in actions {
            if let Some(stats) = self.xdp.get(action) {
                write!(
                    f,
                    "  Action: {}\n    RX Packets: {}\n    RX Bytes: {}\n",
                    action, stats.rx_packets, stats.rx_bytes
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagStats {
    pub tags: HashMap<String, PktStats>,
}

impl fmt::Display for TagStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tags.is_empty() {
            return write!(f, "No tag statistics available.");
        }

        writeln!(f, "Tag Statistics:")?;

        let mut tag_names: Vec<_> = self.tags.keys().collect();
        tag_names.sort();

        for tag_name in tag_names {
            if let Some(stats) = self.tags.get(tag_name) {
                writeln!(
                    f,
                    "  Tag: {}\n    RX Packets: {}\n    RX Bytes: {}",
                    tag_name, stats.rx_packets, stats.rx_bytes
                )?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::indexing_slicing)]
mod tests {
    use super::*;

    #[test]
    fn test_pktstats_default() {
        let stats = PktStats::default();
        assert_eq!(stats.rx_packets, 0);
        assert_eq!(stats.rx_bytes, 0);
    }

    #[test]
    fn test_pktstats_serde_roundtrip() {
        let stats = PktStats {
            rx_packets: 1000,
            rx_bytes: 50000,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: PktStats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.rx_packets, 1000);
        assert_eq!(deserialized.rx_bytes, 50000);
    }

    #[test]
    fn test_stats_display() {
        let mut xdp = HashMap::new();
        xdp.insert(
            "drop".to_string(),
            PktStats {
                rx_packets: 100,
                rx_bytes: 5000,
            },
        );
        xdp.insert(
            "pass".to_string(),
            PktStats {
                rx_packets: 200,
                rx_bytes: 10000,
            },
        );
        let stats = Stats {
            drop_cidr_count: 10,
            ignore_cidr_count: 5,
            xdp,
        };
        let display = stats.to_string();
        assert!(display.contains("Drop CIDR Count: 10"));
        assert!(display.contains("Ignore CIDR Count: 5"));
        assert!(display.contains("Action: drop"));
        assert!(display.contains("Action: pass"));
        assert!(display.contains("RX Packets: 100"));
        assert!(display.contains("RX Bytes: 5000"));
    }

    #[test]
    fn test_stats_display_empty_xdp() {
        let stats = Stats {
            drop_cidr_count: 0,
            ignore_cidr_count: 0,
            xdp: HashMap::new(),
        };
        let display = stats.to_string();
        assert!(display.contains("Drop CIDR Count: 0"));
        assert!(display.contains("Ignore CIDR Count: 0"));
        assert!(display.contains("XDP Stats:"));
    }

    #[test]
    fn test_stats_serde_roundtrip() {
        let mut xdp = HashMap::new();
        xdp.insert(
            "drop".to_string(),
            PktStats {
                rx_packets: 100,
                rx_bytes: 5000,
            },
        );
        let stats = Stats {
            drop_cidr_count: 10,
            ignore_cidr_count: 5,
            xdp,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: Stats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.drop_cidr_count, 10);
        assert_eq!(deserialized.ignore_cidr_count, 5);
        assert_eq!(deserialized.xdp["drop"].rx_packets, 100);
    }

    #[test]
    fn test_tagstats_display_empty() {
        let tag_stats = TagStats {
            tags: HashMap::new(),
        };
        let display = tag_stats.to_string();
        assert!(display.contains("No tag statistics available"));
    }

    #[test]
    fn test_tagstats_display_with_data() {
        let mut tags = HashMap::new();
        tags.insert(
            "malware".to_string(),
            PktStats {
                rx_packets: 50,
                rx_bytes: 2500,
            },
        );
        tags.insert(
            "scanner".to_string(),
            PktStats {
                rx_packets: 30,
                rx_bytes: 1500,
            },
        );
        let tag_stats = TagStats { tags };
        let display = tag_stats.to_string();
        assert!(display.contains("Tag Statistics:"));
        assert!(display.contains("Tag: malware"));
        assert!(display.contains("Tag: scanner"));
        assert!(display.contains("RX Packets: 50"));
        assert!(display.contains("RX Bytes: 2500"));
    }

    #[test]
    fn test_tagstats_serde_roundtrip() {
        let mut tags = HashMap::new();
        let tag = "test-tag".to_string();
        tags.insert(
            tag.clone(),
            PktStats {
                rx_packets: 42,
                rx_bytes: 1234,
            },
        );
        let tag_stats = TagStats { tags };
        let json = serde_json::to_string(&tag_stats).unwrap();
        let deserialized: TagStats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tags[&tag].rx_packets, 42);
        assert_eq!(deserialized.tags[&tag].rx_bytes, 1234);
    }
}
