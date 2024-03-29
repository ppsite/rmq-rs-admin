use serde::Deserialize;
use std::{collections::HashMap, error::Error};

use super::client::Client;

#[derive(Deserialize, Debug)]
pub struct MetaData {
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Vhost {
    pub name: String,
    pub cluster_state: HashMap<String, String>,
    pub default_queue_type: String,
    pub description: String,
    pub metadata: MetaData,
    pub tags: Vec<String>,
    pub tracing: bool,
}

pub struct VhostManager {
    client: Box<Client>,
}

impl VhostManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Vec<Vhost>, Box<dyn Error>> {
        let uri = "api/vhosts".to_string();
        let vhosts = self
            .client
            .get(uri, None)
            .await?
            .json::<Vec<Vhost>>()
            .await?;
        Ok(vhosts)
    }
}
