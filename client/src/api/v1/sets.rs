use crate::{CouicClient, CouicError};
use common::{Policy, Set, SetName, SetSummary};

pub struct SetsApi<'a> {
    client: &'a CouicClient,
}

impl<'a> SetsApi<'a> {
    pub(crate) const fn new(client: &'a CouicClient) -> Self {
        Self { client }
    }

    pub fn list(&self, policy: Policy) -> Result<Vec<SetSummary>, CouicError> {
        self.client.get(&format!("/v1/sets/{policy}"))
    }

    pub fn get(&self, policy: Policy, name: &SetName) -> Result<Set, CouicError> {
        self.client.get(&format!("/v1/sets/{policy}/{name}"))
    }

    pub fn create(&self, policy: Policy, request: &Set) -> Result<Set, CouicError> {
        self.client
            .post(&format!("/v1/sets/{policy}"), Some(request))
    }

    pub fn update(&self, policy: Policy, name: &SetName, set: &Set) -> Result<Set, CouicError> {
        self.client
            .put(&format!("/v1/sets/{policy}/{name}"), Some(set))
    }

    pub fn delete(&self, policy: Policy, name: &SetName) -> Result<(), CouicError> {
        self.client.delete(&format!("/v1/sets/{policy}/{name}"))
    }

    pub fn reload(&self) -> Result<(), CouicError> {
        self.client.post_empty("/v1/sets/reload")
    }
}
