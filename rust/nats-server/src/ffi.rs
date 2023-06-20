// use std::ffi::{CStr, CString}
// use std::os::raw::c_char;

// the functions exported by the go module
extern "C" {
    fn StartNats();
}

pub(crate) fn start_nats() {
    unsafe {
        StartNats();
    }
}