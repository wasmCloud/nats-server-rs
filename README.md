# NATS Rust Server
This is a thin veneer over top of the NATS Go server. This crate is designed to be used by Rust applications and libraries that want to be able to start/stop an embedded NATS server, as shown in the following code:

```rust
let tbd = write_this();
```