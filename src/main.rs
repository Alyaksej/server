mod server;
mod test_client;

use tokio::net::UnixDatagram;
use std::{fs, io};
use std::os::raw::{c_int};
use std::time::Instant;

extern crate libc;

extern {
    fn array_processing(
        data: *mut u8,
        data_max_len: c_int,
        data_used_len: *mut c_int,
        result_out: *mut u8,
        result_max_len: c_int,
        result_used_len: *mut c_int,
    );
}

#[tokio::main]
async fn main() -> io::Result<()> {
    server::run_server();
    Ok(())
}
