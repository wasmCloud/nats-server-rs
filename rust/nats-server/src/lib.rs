mod ffi;

pub struct NatsServer {}

impl NatsServer {
    pub fn new() -> NatsServer {
        NatsServer {  }
    }

    // TODO - convert this to a result return type
    pub fn start(&self) {
        ffi::start_nats();
    }
}