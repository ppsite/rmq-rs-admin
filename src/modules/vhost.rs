use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Default)]
pub struct VhostManager {
    client: Client,
}

impl VhostManager {
    pub fn new(client: Client) -> VhostManager {
        VhostManager { client }
    }

    pub async fn get(&self) -> Vec<Vhost> {
        let uri = "api/vhosts".to_string();
        self.client
            .get(uri, None)
            .await
            .unwrap()
            .json::<Vec<Vhost>>()
            .await
            .unwrap()
    }

    // #[allow(dead_code)]
    // #[test]
    // async fn test_get() {
    //     let client = Client::default();
    //     let vhost_manager = VhostManager::new(client);
    //     let vhosts = vhost_manager.get().await;
    //     assert_eq!(vhosts.len() >= 1, true);
    // }
}
