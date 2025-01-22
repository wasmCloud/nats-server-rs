// the functions exported by the go module
extern "C" {
    fn StartNats();
}

pub(crate) fn start_nats() {
    unsafe {
        StartNats();
    }
}
