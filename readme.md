# Rabbitmq Admin HTTP API Client

> WIP: Resource GET Request

In some cases, you may need to interact with the RabbitMQ HTTP API. This library provides a simple way to interact with the API.

## Usage

```yaml
version: "3"
services:
  rabbitmq:
    image: rabbitmq:3.12.12-management
    container_name: rabbitmq
    restart: always
    privileged: true
    ports:
      - 5672:5672
      - 15672:15672
    environment:
      RABBITMQ_DEFAULT_USER: guest
      RABBITMQ_DEFAULT_PASS: guest
```

```shell
# run rabbitmq container with docker-compose
docker-compose up -d
```

```rust
// This example can be found in `main.rs`
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
    let vhosts = rmq.vhost.get().await?;
    println!("vhosts: {:?}", vhosts);
    let queues = rmq.queue.get().await?;
    println!("queues: {:?}", queues);

    Ok(())
}
```

## Processing

| API | Method | State |
| --- | --- | --- |
| /api/overview | GET |  |
| /api/cluster-name | GET | DONE |
| | PUT |  |
| /api/nodes | GET | DONE |
| /api/nodes/name | GET |  |
| /api/extensions | GET |  |
| /api/definitions | GET |  |
| | POST |  |
| /api/all-configuration (deprecated) | GET |  |
| /api/definitions/vhost | GET | None |
| | POST |  |
| /api/connections | GET |  |
| /api/vhosts/vhost/connections | GET |  |
| /api/connections/name | GET |  |
| | DELETE |  |
| /api/connections/name/channels | GET |  |
| /api/channels | GET |  |
| /api/vhosts/vhost/channels | GET |  |
| /api/channels/channel | GET |  |
| /api/consumers | GET |  |
| /api/consumers/vhost | GET |  |
| /api/exchanges | GET |  |
| /api/exchanges/vhost | GET |  |
| /api/exchanges/vhost/name | GET |  |
| /api/exchanges/vhost/name | PUT |  |
| /api/exchanges/vhost/name | DELETE |  |
| /api/exchanges/vhost/name/bindings/source | GET |  |
| /api/exchanges/vhost/name/bindings/destination | GET |  |
| /api/exchanges/vhost/name/publish | POST |  |
| /api/queues | GET | Done |
| /api/queues/vhost | GET |  |
| /api/queues/vhost/name | GET |  |
| /api/queues/vhost/name | PUT |  |
| /api/queues/vhost/name | DELETE |  |
| /api/queues/vhost/name/bindings | GET |  |
| /api/queues/vhost/name/contents | GET |  |
| /api/queues/vhost/name/actions | POST |  |
| /api/queues/vhost/name/get | POST |  |
| /api/bindings | GET |  |
| /api/bindings/vhost | GET |  |
| /api/bindings/vhost/e/exchange/q/queue | GET |  |
| /api/bindings/vhost/e/exchange/q/queue/props | GET |  |
| /api/bindings/vhost/e/source/e/destination | GET |  |
| /api/bindings/vhost/e/source/e/destination/props | GET |  |
| /api/vhosts | GET |  |
| /api/vhosts/name | GET |  |
| /api/vhosts/name/permissions | GET |  |
| /api/vhosts/name/topic-permissions | GET |  |
| /api/vhosts/name/start/node | GET |  |
| /api/users/ | GET |  |
| /api/users/without-permissions | GET |  |
| /api/users/bulk-delete | POST |  |
| /api/users/name | GET |  |
| /api/users/user/permissions | GET |  |
| /api/users/user/topic-permissions | GET |  |
| /api/user-limits | GET |  |
| /api/user-limits/user | GET |  |
| /api/user-limits/user/name | GET |  |
| /api/whoami | GET |  |
| /api/permissions | GET |  |
| /api/permissions/vhost/user | GET |  |
| /api/topic-permissions | GET |  |
| /api/topic-permissions/vhost/user | GET |  |
| /api/parameters | GET |  |
| /api/parameters/component | GET |  |
| /api/parameters/component/vhost | GET |  |
| /api/parameters/component/vhost/name | GET |  |
| /api/global-parameters | GET |  |
| /api/global-parameters/name | GET |  |
| /api/policies | GET |  |
| /api/policies/vhost | GET |  |
| /api/policies/vhost/name | GET |  |
| /api/operator-policies | GET |  |
| /api/operator-policies/vhost | GET |  |
| /api/operator-policies/vhost/name | GET |  |
| /api/aliveness-test/vhost | GET |  |
| /api/health/checks/alarms | GET |  |
| /api/health/checks/local-alarms | GET |  |
| /api/health/checks/certificate-expiration/within/unit | GET |  |
| /api/health/checks/port-listener/port | GET |  |
| /api/health/checks/protocol-listener/protocol | GET |  |
| /api/health/checks/virtual-hosts | GET |  |
| /api/health/checks/node-is-mirror-sync-critical | GET |  |
| /api/health/checks/node-is-quorum-critical | GET |  |
| /api/vhost-limits | GET |  |
| /api/vhost-limits/vhost | GET |  |
| /api/vhost-limits/vhost/name | GET |  |
| /api/auth | GET |  |
| /api/rebalance/queues | POST |  |
| /api/federation-links | GET |  |
| /api/federation-links/vhost | GET |  |
| /api/auth/attempts/node | GET |  |
| /api/auth/attempts/node/source | GET |  |
