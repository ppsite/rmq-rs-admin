#[warn(dead_code)]
use rmq_rs_admin::Rabbitmq;

#[tokio::main]
async fn main() {
    let rmq = Rabbitmq::default();
    let vhosts = rmq.vhost.get().await;
    println!("vhosts: {:?}", vhosts);
}
