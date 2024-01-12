use serde::{Deserialize, Serialize};
use std::error::Error;

use super::{client::Client, utils::RateDetails};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionDetails {
    pub name: String,
    pub peer_host: String,
    pub peer_port: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GarbageCollection {
    pub fullsweep_after: u32,
    pub max_heap_size: u32,
    pub min_bin_vheap_size: u32,
    pub min_heap_size: u32,
    pub minor_gcs: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStats {
    pub confirm: u32,
    pub confirm_details: RateDetails,
    pub drop_unroutable: u32,
    pub drop_unroutable_details: RateDetails,
    pub publish: u32,
    pub publish_details: RateDetails,
    pub return_unroutable: u32,
    pub return_unroutable_details: RateDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub acks_uncommitted: u32,
    pub confirm: bool,
    pub connection_details: ConnectionDetails,
    pub consumer_count: u32,
    pub garbage_collection: GarbageCollection,
    pub global_prefetch_count: u32,
    pub message_stats: MessageStats,
    pub messages_unacknowledged: u32,
    pub messages_uncommitted: u32,
    pub messages_unconfirmed: u32,
    pub name: String,
    pub node: String,
    pub number: u32,
    pub pending_raft_commands: u32,
    pub prefetch_count: u32,
    pub reductions: u32,
    pub reductions_details: RateDetails,
    pub state: String,
    pub transactional: bool,
    pub user: String,
    pub user_who_performed_action: String,
    pub vhost: String,
}

pub struct ChannelManager {
    client: Box<Client>,
}

impl ChannelManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Vec<Channel>, Box<dyn Error>> {
        let uri = "api/channels".to_string();
        let objects = self
            .client
            .get(uri, None)
            .await?
            // .text_with_charset("utf-8")
            .json::<Vec<Channel>>()
            .await?;
        Ok(objects)
    }
}
