use crate::{CouicClient, CouicError};
use common::{Policy, Stats, TagStats};

pub struct StatsApi<'a> {
    client: &'a CouicClient,
}

impl<'a> StatsApi<'a> {
    pub(crate) const fn new(client: &'a CouicClient) -> Self {
        Self { client }
    }

    pub fn get(&self) -> Result<Stats, CouicError> {
        self.client.get("/v1/stats")
    }

    pub fn tag(&self, policy: Policy) -> Result<TagStats, CouicError> {
        self.client.get(&format!("/v1/stats/tags/{policy}"))
    }
}
