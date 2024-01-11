use serde::{Deserialize, Serialize};
use std::error::Error;

use super::client::Client;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Node {
    pub partitions: Vec<String>,
    pub os_pid: String,
    pub fd_total: i64,
    pub sockets_total: i64,
    pub mem_limit: i64,
    pub mem_alarm: bool,
    pub disk_free_limit: i64,
    pub disk_free_alarm: bool,
    pub proc_total: i64,
    pub rates_mode: String,
    pub uptime: i64,
    pub run_queue: i32,
    pub processors: i32,
    pub exchange_types: Vec<ExchangeType>,
    pub auth_mechanisms: Vec<AuthMechanism>,
    pub applications: Vec<Application>,
    pub contexts: Vec<Context>,
    pub log_files: Vec<String>,
    pub db_dir: String,
    pub config_files: Vec<String>,
    pub net_ticktime: i32,
    pub enabled_plugins: Vec<String>,
    pub mem_calculation_strategy: String,
    pub ra_open_file_metrics: HashMap<String, i32>,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub running: bool,
    pub being_drained: bool,
    pub mem_used: i64,
    pub mem_used_details: RateDetails,
    pub fd_used: i32,
    pub fd_used_details: RateDetails,
    pub sockets_used: i32,
    pub sockets_used_details: RateDetails,
    pub proc_used: i32,
    pub proc_used_details: RateDetails,
    pub disk_free: i64,
    pub disk_free_details: RateDetails,
    pub gc_num: i64,
    pub gc_num_details: RateDetails,
    pub gc_bytes_reclaimed: i64,
    pub gc_bytes_reclaimed_details: RateDetails,
    pub context_switches: i64,
    pub context_switches_details: RateDetails,
    pub io_read_count: i64,
    pub io_read_count_details: RateDetails,
    pub io_read_bytes: i64,
    pub io_read_bytes_details: RateDetails,
    pub io_read_avg_time: f64,
    pub io_read_avg_time_details: RateDetails,
    pub io_write_count: i64,
    pub io_write_count_details: RateDetails,
    pub io_write_bytes: i64,
    pub io_write_bytes_details: RateDetails,
    pub io_write_avg_time: f64,
    pub io_write_avg_time_details: RateDetails,
    pub io_sync_count: i64,
    pub io_sync_count_details: RateDetails,
    pub io_sync_avg_time: f64,
    pub io_sync_avg_time_details: RateDetails,
    pub io_seek_count: i64,
    pub io_seek_count_details: RateDetails,
    pub io_seek_avg_time: f64,
    pub io_seek_avg_time_details: RateDetails,
    pub io_reopen_count: i64,
    pub io_reopen_count_details: RateDetails,
    pub mnesia_ram_tx_count: i64,
    pub mnesia_ram_tx_count_details: RateDetails,
    pub mnesia_disk_tx_count: i64,
    pub mnesia_disk_tx_count_details: RateDetails,
    pub msg_store_read_count: i64,
    pub msg_store_read_count_details: RateDetails,
    pub msg_store_write_count: i64,
    pub msg_store_write_count_details: RateDetails,
    pub queue_index_write_count: i64,
    pub queue_index_write_count_details: RateDetails,
    pub queue_index_read_count: i64,
    pub queue_index_read_count_details: RateDetails,
    pub connection_created: i64,
    pub connection_created_details: RateDetails,
    pub connection_closed: i64,
    pub connection_closed_details: RateDetails,
    pub channel_created: i64,
    pub channel_created_details: RateDetails,
    pub channel_closed: i64,
    pub channel_closed_details: RateDetails,
    pub queue_declared: i64,
    pub queue_declared_details: RateDetails,
    pub queue_created: i64,
    pub queue_created_details: RateDetails,
    pub queue_deleted: i64,
    pub queue_deleted_details: RateDetails,
    pub cluster_links: Vec<ClusterLink>,
    pub metrics_gc_queue_length: MetricsGcQueueLength,
}

#[derive(Deserialize, Debug)]
pub struct ExchangeType {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct AuthMechanism {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct Context {
    pub description: String,
    pub path: String,
    pub cowboy_opts: String,
    pub port: String,
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RateDetails {
    pub rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsGcQueueLength {
    pub connection_closed: i32,
    pub channel_closed: i32,
    pub consumer_deleted: i32,
    pub exchange_deleted: i32,
    pub queue_deleted: i32,
    pub vhost_deleted: i32,
    pub node_node_deleted: i32,
    pub channel_consumer_deleted: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClusterLink {
    pub stats: LinkStats,
    pub name: String,
    pub peer_addr: String,
    pub peer_port: u16,
    pub sock_addr: String,
    pub sock_port: u16,
    pub recv_bytes: u64,
    pub send_bytes: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinkStats {
    pub send_bytes: u64,
    pub send_bytes_details: RateDetails,
    pub recv_bytes: u64,
    pub recv_bytes_details: RateDetails,
}

pub struct NodeManager {
    client: Box<Client>,
}

impl NodeManager {
    pub fn new(client: Box<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Vec<Node>, Box<dyn Error>> {
        let uri = "api/nodes".to_string();
        let nodes = self
            .client
            .get(uri, None)
            .await?
            .json::<Vec<Node>>()
            .await?;
        Ok(nodes)
    }
}
