// NOTE: for demo purposes if you're using dynamic linking, for 
// now you have to copy the librustnats file into the same directory
// as the executable to run it.  This will be fixed in the future.
fn main() {
    let server = nats_server::NatsServer::new();

    println!("Running NATS from Rust!");
    server.start();
}
