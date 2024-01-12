use serde::{Deserialize, Serialize};
use std::error::Error;

use super::client::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
    pub javascript: String,
}

pub struct ExtensionManager {
    client: Box<Client>,
}

impl ExtensionManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Vec<Extension>, Box<dyn Error>> {
        let uri = "api/extensions".to_string();
        let objects = self
            .client
            .get(uri, None)
            .await?
            .json::<Vec<Extension>>()
            .await?;
        Ok(objects)
    }
}
