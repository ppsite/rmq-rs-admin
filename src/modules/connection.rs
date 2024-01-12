use serde::{Deserialize, Serialize};
use std::error::Error;

use super::{client::Client, utils::RateDetails};

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub auth_mechanism: String,
    pub channel_max: u32,
    pub channels: u32,
    pub client_properties: ClientProperties,
    pub connected_at: u64,
    pub frame_max: u32,
    pub garbage_collection: GarbageCollection,
    pub host: String,
    pub name: String,
    pub node: String,
    pub peer_cert_issuer: Option<String>,
    pub peer_cert_subject: Option<String>,
    pub peer_cert_validity: Option<String>,
    pub peer_host: String,
    pub peer_port: u32,
    pub port: u32,
    pub protocol: String,
    pub recv_cnt: u32,
    pub recv_oct: u32,
    pub recv_oct_details: RateDetails,
    pub reductions: u32,
    pub reductions_details: RateDetails,
    pub send_cnt: u32,
    pub send_oct: u32,
    pub send_oct_details: RateDetails,
    pub send_pend: u32,
    pub ssl: bool,
    pub ssl_cipher: Option<String>,
    pub ssl_hash: Option<String>,
    pub ssl_key_exchange: Option<String>,
    pub ssl_protocol: Option<String>,
    pub state: String,
    pub timeout: u32,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub user: String,
    pub user_who_performed_action: String,
    pub vhost: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientProperties {
    pub capabilities: Capabilities,
    pub information: String,
    pub platform: String,
    pub product: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub authentication_failure_close: bool,
    #[serde(skip)]
    pub basic_nack: bool,
    #[serde(skip)]
    pub connection_blocked: bool,
    pub consumer_cancel_notify: bool,
    pub publisher_confirms: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GarbageCollection {
    pub fullsweep_after: u32,
    pub max_heap_size: u32,
    pub min_bin_vheap_size: u32,
    pub min_heap_size: u32,
    pub minor_gcs: u32,
}

pub struct ConnectionManager {
    client: Box<Client>,
}

impl ConnectionManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Vec<Connection>, Box<dyn Error>> {
        let uri = "api/connections".to_string();
        let objects = self
            .client
            .get(uri, None)
            .await?
            .json::<Vec<Connection>>()
            .await?;
        Ok(objects)
    }
}
