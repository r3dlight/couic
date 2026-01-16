use crate::{CouicClient, CouicError};
use common::PeerJob;

pub struct PeerApi<'a> {
    client: &'a CouicClient,
}

impl<'a> PeerApi<'a> {
    pub(crate) const fn new(client: &'a CouicClient) -> Self {
        Self { client }
    }

    pub fn drop(&self, jobs: &[PeerJob]) -> Result<Vec<PeerJob>, CouicError> {
        self.client.post("/v1/drop/peer", Some(jobs))
    }
}
