use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use super::client::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageStats {
    pub publish: u32,
    pub publish_details: HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Queue {
    pub arguments: HashMap<String, String>,
    pub auto_delete: bool,
    pub consumer_capacity: u32,
    pub consumer_utilisation: f32,
    pub consumers: u16,
    pub durable: bool,
    pub exclusive: bool,
    pub exclusive_consumer_tag: Option<String>,
    pub garbage_collection: HashMap<String, u32>,
    pub head_message_timestamp: Option<String>,
    pub idle_since: Option<String>,
    pub memory: u32,
    pub message_bytes: u32,
    pub message_bytes_paged_out: u32,
    pub message_bytes_persistent: u32,
    pub message_bytes_ram: u32,
    pub message_bytes_ready: u32,
    pub message_bytes_unacknowledged: u32,
    pub message_stats: MessageStats,
    pub messages_paged_out: u32,
    pub messages_persistent: u32,
    pub messages_ram: u32,
    pub messages_ready: u32,
    pub messages_ready_details: HashMap<String, f32>,
    pub messages_ready_ram: u32,
    pub messages_unacknowledged: u32,
    pub messages_unacknowledged_details: HashMap<String, f32>,
    pub messages_unacknowledged_ram: u32,
    pub name: String,
    pub node: String,
    pub operator_policy: Option<String>,
    pub policy: String,
    pub recoverable_slaves: Option<String>,
    pub reductions: u32,
    pub reductions_details: HashMap<String, f32>,
    pub state: String,
    pub vhost: String,
    pub single_active_consumer_tag: Option<String>,
}

pub struct QueueManager {
    client: Box<Client>,
}

impl QueueManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Vec<Queue>, Box<dyn Error>> {
        let uri = "api/queues".to_string();
        let objects = self
            .client
            .get(uri, None)
            .await?
            .json::<Vec<Queue>>()
            .await?;
        Ok(objects)
    }
}
