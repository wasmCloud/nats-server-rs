> [!WARNING]
> This repository is archived and is no longer maintained. It receives no bug fixes or security updates.
> The source is retained for historical reference. There is no maintained replacement for this embedded NATS server wrapper in the wasmCloud organization.

# NATS Rust Server
This is a thin veneer over top of the NATS Go server. This crate is designed to be used by Rust applications and libraries that want to be able to start/stop an embedded NATS server, as shown in the following code:

```rust
fn main() {
    let server = nats_server::NatsServer::new();

    println!("Running NATS from Rust!");
    server.start();
}
```
