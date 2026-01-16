use crate::{CouicClient, CouicError};
use common::{Entry, Policy, RawEntry};

pub struct PolicyApi<'a> {
    client: &'a CouicClient,
}

impl<'a> PolicyApi<'a> {
    pub(crate) const fn new(client: &'a CouicClient) -> Self {
        Self { client }
    }

    pub fn get(&self, policy: Policy, cidr: &str) -> Result<Entry, CouicError> {
        self.client.get(&format!("/v1/{policy}/{cidr}"))
    }

    pub fn list(&self, policy: Policy) -> Result<Vec<Entry>, CouicError> {
        self.client.get(&format!("/v1/{policy}"))
    }

    pub fn add(&self, policy: Policy, entry: &RawEntry) -> Result<Entry, CouicError> {
        self.client.post(&format!("/v1/{policy}"), Some(entry))
    }

    pub fn delete(&self, policy: Policy, cidr: &str) -> Result<(), CouicError> {
        self.client.delete(&format!("/v1/{policy}/{cidr}"))
    }
}
