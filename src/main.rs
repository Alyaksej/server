use tokio::net::UnixDatagram;
use tokio::io::Interest;
use std::{fs, io};
use std::time::Instant;
use libc::connect;

extern crate libc;

#[tokio::main]
async fn main() -> io::Result<()> {
    const SOCKET_IN_PATH: &str = "/tmp/socket_in.sock";
    const SOCKET_OUT_PATH: &str = "/tmp/socket_out.sock";
    const BUFFER_SIZE: usize = 1_000_000;
    const BUFFER_THRESHOLD: usize = BUFFER_SIZE - 200_000;
    // Remove socket before start
    if fs::metadata(SOCKET_IN_PATH).is_ok() {
        if let Err(e) = fs::remove_file(SOCKET_IN_PATH) {
            eprintln!("Error removing socket file: {}", e);
            return Err(e);
        }
    };
    // if fs::metadata(SOCKET_OUT_PATH).is_ok() {
    //     if let Err(e) = fs::remove_file(SOCKET_OUT_PATH) {
    //         eprintln!("Error removing socket file: {}", e);
    //         return Err(e);
    //     }
    // };
    // Create socket
    let socket_in = match UnixDatagram::bind(SOCKET_IN_PATH) {
        Ok(socket_in) => socket_in,
        Err(e) => {
            eprintln!("Error binding socket: {}", e);
            return Err(e);
        }
    };

    // let socket_out = match UnixDatagram::bind(SOCKET_OUT_PATH){
    //     Ok(socket_out) => socket_out,
    //     Err(e) => {
    //         eprintln!("Error binding socket: {}", e);
    //         return Err(e);
    //     }
    // };

    let mut cnt_recv = 0;
    let mut whole_bytes = 0;
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut buffer_offset: usize = 0;
    let buf: [u8; 5] = [1, 2, 3, 4, 5];

    let mut now = Instant::now();
    let time = Instant::now();

    loop {
        if buffer_offset >= BUFFER_THRESHOLD {
            buffer_offset = 0;
        }
        let body_slice: &mut [u8] = &mut buffer[buffer_offset..];
        //let _ = socket_in.readable().await;
        let ready = socket_in.ready(Interest::READABLE | Interest::WRITABLE).await?;
        //let _ = socket_out.writable().await;
        if ready.is_readable() {
            match socket_in.try_recv(body_slice) {
                Ok(len_recv) => {
                    if len_recv > body_slice.len() {
                        println!("Error receiving data: data is to long");
                        return Err(io::Error::new(io::ErrorKind::Other, "Error receiving data: data is to long"));
                    };
                    buffer_offset += len_recv;
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
        unsafe {
            //let _result = byteToInt(lib_ptr, lib_len_max);
            // for i in 0..MAX_NUMBERS {
            //     println!("result: {}", *result.offset(i.try_into().unwrap()));
            // }
            // if ready.is_writable() {
            //     match socket_in.try_send_to(&buf, &SOCKET_IN_PATH) {
            //         Ok(n) => {
            //             println!("!!!!!!!!!!!!!!!n: {} {:?}", n, buf)
            //         }
            //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            //             continue;
            //         }
            //         Err(e) => {
            //             return Err(e);
            //         }
            //     }
            // }
        }

        if now.elapsed().as_secs() >= 1 {
            server_bandwidth(cnt_recv, &mut whole_bytes, time);
            cnt_recv = 0;
            now = Instant::now();
            //if fs::metadata(SOCKET_OUT_PATH).is_ok() {
                let client_socket = UnixDatagram::unbound().unwrap();
                let ready = client_socket.ready(Interest::WRITABLE).await?;
                if ready.is_writable() {
                    match client_socket.try_send_to(&buf, &SOCKET_OUT_PATH) {
                        Ok(n) => {
                            println!("!!!!!!!!!!!!!!!n: {} {:?}", n, buf)
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            println!("Connection SOCKET_OUT refused");
                            continue;
                        }
                    }
                }
            //}
        }
    }
    Ok(())
}

fn server_bandwidth(cnt_bytes: usize, whole_bytes: &mut usize, time: Instant) {
    *whole_bytes += cnt_bytes;
    println!("{} MB/sec\n{} MB total\n{} secs total work time\
    \n________________", cnt_bytes / 1_000_000, *whole_bytes / 1_000_000, time.elapsed().as_secs());
}
