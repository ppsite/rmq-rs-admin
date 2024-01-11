use std::error::Error;

#[warn(dead_code)]
use rmq_rs_admin::Rabbitmq;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rmq = Rabbitmq::new(
        "http://127.0.0.1".to_string(),
        15672,
        "Z3Vlc3Q6Z3Vlc3Q=".to_string(),
        5,
    );
    // let vhosts = rmq.vhost.get().await?;
    // println!("vhosts: {:?}", vhosts);
    // let queues = rmq.queue.get().await?;
    // println!("queues: {:?}", queues);

    let cluster_name = rmq.cluster_name.get().await?;
    println!("cluster_name: {:?}", cluster_name);

    let nodes = rmq.node.get().await?;
    println!("nodes: {:?}", nodes);

    Ok(())
}
