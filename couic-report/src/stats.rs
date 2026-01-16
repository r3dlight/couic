use std::collections::{HashMap, HashSet};

pub use common::Report;

#[derive(Debug, Clone)]
pub struct Statistics {
    pub total_count: usize,
    pub distinct_cidrs: usize,
    pub top_tag: Option<(String, usize)>,
}

#[derive(Debug, Default)]
pub struct StatisticsAccumulator {
    pub total_count: usize,
    cidrs: HashSet<String>,
    tag_counts: HashMap<String, usize>,
}

impl StatisticsAccumulator {
    pub fn add_reports(&mut self, reports: &[Report]) {
        self.total_count += reports.len();

        for report in reports {
            self.cidrs.insert(report.entry.cidr.to_string());
            if let Some(tag) = &report.entry.tag {
                *self.tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
    }

    pub fn to_statistics(&self) -> Statistics {
        let top_tag = self
            .tag_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(tag, count)| (tag.clone(), *count));

        Statistics {
            total_count: self.total_count,
            distinct_cidrs: self.cidrs.len(),
            top_tag,
        }
    }

    pub fn reset(&mut self) {
        self.total_count = 0;
        self.cidrs.clear();
        self.tag_counts.clear();
    }
}
