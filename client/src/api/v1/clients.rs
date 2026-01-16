use crate::{CouicClient, CouicError};
use common::{Client, ClientName, ClientRaw};

pub struct ClientsApi<'a> {
    client: &'a CouicClient,
}

impl<'a> ClientsApi<'a> {
    pub(crate) const fn new(client: &'a CouicClient) -> Self {
        Self { client }
    }

    pub fn get(&self, name: &ClientName) -> Result<Client, CouicError> {
        self.client.get(&format!("/v1/client/{name}"))
    }

    pub fn list(&self) -> Result<Vec<Client>, CouicError> {
        self.client.get("/v1/client")
    }

    pub fn add(&self, request: &ClientRaw) -> Result<Client, CouicError> {
        self.client.post("/v1/client", Some(request))
    }

    pub fn delete(&self, name: &ClientName) -> Result<(), CouicError> {
        self.client.delete(&format!("/v1/client/{name}"))
    }
}
