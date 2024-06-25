use tokio::net::UnixDatagram;
use tokio::io::Interest;
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
    const SOCKET_DATA_PATH: &str = "/tmp/socket_data.sock";
    const SOCKET_RESULT_PATH: &str = "/tmp/socket_result.sock";
    const DATA_SIZE: usize = 2000_000_000;
    const RESULT_SIZE: usize = 1000_000;
    const BUFFER_THRESHOLD: usize = DATA_SIZE - 200_000;
    const STATS_PERIOD: u64 = 5;
    const MBYTES: u64 = 1_000_000;
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

    let mut data_vec = vec![0; DATA_SIZE];
    let data_c_ptr = data_vec.as_mut_ptr();
    let mut data_offset: usize = 0;

    let mut result_vec = vec![0; RESULT_SIZE];
    let result_c_ptr = result_vec.as_mut_ptr();
    let result_max_len = 200_000;

    let mut cnt_bytes = 0;
    let mut whole_bytes = 0;
    let mut stats_start_ts = Instant::now();
    let server_start_ts = Instant::now();

    let mut next_pkt_num = 0;

    loop {
        if data_offset >= BUFFER_THRESHOLD {
            println!("{:?}", data_vec);
            data_offset = 0;
        }
        let data_free_slice: &mut [u8] = &mut data_vec[data_offset..];
        //let _ = socket_data.readable().await;
        let ready = socket_data.ready(Interest::READABLE).await?;
        //let _ = socket_result.writable().await;
        if ready.is_readable() {
            match socket_data.try_recv(data_free_slice) {
                Ok(len_recv) => {
                    if len_recv >= data_free_slice.len() {
                        return Err(io::Error::new(io::ErrorKind::Other, "Error receiving data: data is to long"));
                    };
                    let recv_pkt_num = data_free_slice[0];
                    if recv_pkt_num != next_pkt_num {
                        println!("!!!!! ERROR recv_pkt_num is equal!");
                    }
                    if recv_pkt_num == 255 {
                        next_pkt_num = 0;
                    } else {
                        next_pkt_num = recv_pkt_num + 1;
                    }

                    data_offset += len_recv;
                    cnt_bytes += len_recv;
                }
                Err(ref e) if
                e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    eprintln!("Error receiving data: {:?}", e);
                    return Err(e);
                }
            }
        }
        // Using of C-library
        let mut data_used_len: c_int = 0;
        let mut result_used_len: c_int = 0;
        let data_max_len: c_int = data_offset as c_int;

        unsafe {
            array_processing(
                data_c_ptr,
                data_max_len,
                &mut data_used_len,
                result_c_ptr,
                result_max_len,
                &mut result_used_len,
            );

            if data_used_len > 0 {
                data_vec.copy_within(data_used_len as usize..data_offset, 0);
                data_offset = data_offset - data_used_len as usize;
            }
        }
        let now = Instant::now();
        if (now - stats_start_ts).as_secs() >= STATS_PERIOD {
            whole_bytes += cnt_bytes;
            println!("{} MB/sec {} MB total {} secs total work time ", cnt_bytes as u64 / (MBYTES * STATS_PERIOD), whole_bytes as u64 / (MBYTES * STATS_PERIOD), server_start_ts.elapsed().as_secs());
            cnt_bytes = 0;
            stats_start_ts = now;
        }
    }
    Ok(())
}
