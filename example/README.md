# Rust NATS Server Example
This binary shows how to incorporate the `nats-server` crate (FFI wrapper) in your own application to launch a NATS server.

Currently this requires copying the lib file(s) around, but that'll be fixed asap.

This is what it looks like running on a Mac with `librustnats` in the `target/debug` folder:

```
./example
Running NATS from Rust!
[75588] 2023/06/20 11:42:10.660355 [INF] Starting nats-server
[75588] 2023/06/20 11:42:10.660707 [INF]   Version:  2.9.18
[75588] 2023/06/20 11:42:10.660711 [INF]   Git:      [not set]
[75588] 2023/06/20 11:42:10.660713 [INF]   Name:     NCTRDM6YXICC3Y6JJKN5ILHZ7A5XNBSWSNFTERWN6IWYRUSXUDVJHVZG
[75588] 2023/06/20 11:42:10.660717 [INF]   ID:       NCTRDM6YXICC3Y6JJKN5ILHZ7A5XNBSWSNFTERWN6IWYRUSXUDVJHVZG
[75588] 2023/06/20 11:42:10.661867 [FTL] Error listening on port: 0.0.0.0:4222, "listen tcp 0.0.0.0:4222: bind: address already in use"
```