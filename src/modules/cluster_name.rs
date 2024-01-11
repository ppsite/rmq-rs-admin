use serde::Deserialize;
use std::error::Error;

use super::client::Client;

#[derive(Deserialize, Debug)]
pub struct ClusterName {
    pub name: String,
}

pub struct ClusterNameManager {
    client: Box<Client>,
}

impl ClusterNameManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<ClusterName, Box<dyn Error>> {
        let uri = "api/cluster-name".to_string();
        let cluster_name = self
            .client
            .get(uri, None)
            .await?
            .json::<ClusterName>()
            .await?;
        Ok(cluster_name)
    }
}
