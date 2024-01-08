use modules::{client::Client, vhost::VhostManager};

pub mod modules;

#[derive(Default)]
pub struct Rabbitmq {
    pub vhost: VhostManager,
}

impl Rabbitmq {
    pub fn new(host: String, port: u16, auth_token: String, timeout: u8) -> Self {
        let client = Client::new(host, port, auth_token, timeout);
        let vhost_manager = VhostManager::new(client);
        Rabbitmq {
            vhost: vhost_manager,
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
