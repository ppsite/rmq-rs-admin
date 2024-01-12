use modules::{
    channel::ChannelManager, client::Client, cluster_name::ClusterNameManager,
    connection::ConnectionManager, node::NodeManager, queue::QueueManager, vhost::VhostManager,
};

pub mod modules;

pub struct Rabbitmq {
    pub vhost: VhostManager,
    pub queue: QueueManager,
    pub cluster_name: ClusterNameManager,
    pub node: NodeManager,
    pub connection: ConnectionManager,
    pub channel: ChannelManager,
}

impl Rabbitmq {
    pub fn new(host: String, port: u16, auth_token: String, timeout: u8) -> Self {
        let client = Client::new(host, port, auth_token, timeout);
        let vhost_manager = VhostManager::new(client.clone());
        let queue_manager = QueueManager::new(client.clone());
        let cluster_name_manager = ClusterNameManager::new(client.clone());
        let node_manager = NodeManager::new(client.clone());
        let connection_manager = ConnectionManager::new(client.clone());
        let channel_manager = ChannelManager::new(client.clone());
        Rabbitmq {
            vhost: vhost_manager,
            queue: queue_manager,
            cluster_name: cluster_name_manager,
            node: node_manager,
            connection: connection_manager,
            channel: channel_manager,
        }
    }
}

// // -----------------------------------------------
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
