use tokio::net::UnixDatagram;
use tokio::io::Interest;
use std::{fs, io};
use std::os::raw::{c_int, c_void};
use std::time::Instant;

extern crate libc;

extern {
    fn array_processing (data: *mut c_void,
                         data_max_len: c_int,
                         data_used_len: *mut c_int,
                         result_out: *mut c_void,
                         result_max_len: c_int,
                         result_used_len: *mut c_int
    );
}

#[tokio::main]
async fn main() -> io::Result<()> {
    const SOCKET_DATA_PATH: &str = "/tmp/socket_data.sock";
    const SOCKET_RESULT_PATH: &str = "/tmp/socket_result.sock";
    const DATA_SIZE: usize = 1_000_000;
    const BUFFER_THRESHOLD: usize = DATA_SIZE - 200_000;
    // Remove socket before start
    if fs::metadata(SOCKET_DATA_PATH).is_ok() {
        if let Err(e) = fs::remove_file(SOCKET_DATA_PATH) {
            eprintln!("Error removing socket file: {}", e);
            return Err(e);
        }
    };
    // if fs::metadata(SOCKET_RESULT_PATH).is_ok() {
    //     if let Err(e) = fs::remove_file(SOCKET_RESULT_PATH) {
    //         eprintln!("Error removing socket file: {}", e);
    //         return Err(e);
    //     }
    // };
    // Create socket
    let socket_data = match UnixDatagram::bind(SOCKET_DATA_PATH) {
        Ok(socket_data) => socket_data,
        Err(e) => {
            eprintln!("Error binding socket data: {}", e);
            return Err(e);
        }
    };

    // let socket_result = match UnixDatagram::bind(SOCKET_OUT_PATH){
    //     Ok(socket_result) => socket_result,
    //     Err(e) => {
    //         eprintln!("Error binding socket result: {}", e);
    //         return Err(e);
    //     }
    // };

    let mut cnt_recv = 0;
    let mut whole_bytes = 0;
    let mut buffer_data = vec![0; DATA_SIZE];
    let mut data_offset: usize = 0;

    let mut now = Instant::now();
    let time = Instant::now();

    loop {
        if data_offset >= BUFFER_THRESHOLD {
            data_offset = 0;
        }
        let body_slice: &mut [u8] = &mut buffer_data[data_offset..];
        //let _ = socket_data.readable().await;
        let ready = socket_data.ready(Interest::READABLE).await?;
        //let _ = socket_result.writable().await;
        if ready.is_readable() {
            match socket_data.try_recv(body_slice) {
                Ok(len_recv) => {
                    if len_recv > body_slice.len() {
                        println!("Error receiving data: data is to long");
                        return Err(io::Error::new(io::ErrorKind::Other, "Error receiving data: data is to long"));
                    };
                    data_offset += len_recv;
                    cnt_recv += len_recv;
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    eprintln!("Error receiving data: {:?}", e);
                    return Err(e);
                }
            }
        }
        // Using of C-library
        let data = buffer_data.as_mut_ptr() as *mut c_void;
        let data_max_len = buffer_data.len() as c_int;
        let mut data_used_len: c_int = 0;
        let mut result_out: *mut c_void = std::ptr::null_mut();
        let mut result_max_len: c_int = 0;
        let result_used_len: *mut c_int = std::ptr::null_mut();;
        unsafe {
            array_processing (data,
                              data_max_len,
                              &mut data_used_len,
                              result_out,
                              result_max_len,
                              result_used_len
            );
        }

        if now.elapsed().as_secs() >= 1 {
            server_bandwidth(cnt_recv, &mut whole_bytes, time);
            cnt_recv = 0;
            now = Instant::now();
        }
    }
    Ok(())
}

fn server_bandwidth(cnt_bytes: usize, whole_bytes: &mut usize, time: Instant) {
    *whole_bytes += cnt_bytes;
    println!("{} MB/sec\n{} MB total\n{} secs total work time\
    \n________________", cnt_bytes / 1_000_000, *whole_bytes / 1_000_000, time.elapsed().as_secs());
}
