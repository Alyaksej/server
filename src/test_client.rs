use tokio::net::UnixDatagram;
use std::{fs, io};

pub(crate) async fn run_test_client() -> io::Result<()> {
    const SOCKET_DATA_PATH: &str = "/app/data-volume/socket_data.sock";
    const SOCKET_RESULT_PATH: &str = "/app/data-volume/socket_result.sock";
    const BUFFER_SIZE: usize = 212_765;
    // Remove socket before start
    if fs::metadata(SOCKET_RESULT_PATH).is_ok() {
        if let Err(e) = fs::remove_file(SOCKET_RESULT_PATH) {
            eprintln!("Error removing socket result file: {}", e);
            return Err(e);
        }
    };
    // Create sockets
    let socket_result = match UnixDatagram::bind(SOCKET_RESULT_PATH) {
        Ok(socket_result) => socket_result,
        Err(e) => {
            eprintln!("Error binding socket result: {}", e);
            return Err(e);
        }
    };

    let socket_data = UnixDatagram::unbound()?;

    let mut buffer = vec![0; BUFFER_SIZE];
    let mut next_pkt_num = 0;
    let mut buf = vec![0; 99_999];

    for i in 0..BUFFER_SIZE {
        buffer[i] = i as u8;
    }
    loop {
        socket_data.writable().await?;

        match socket_data.try_send_to(&buffer, SOCKET_DATA_PATH) {
            Ok(n) => {
                buffer[0] = next_pkt_num;
                if next_pkt_num == 255 {
                    next_pkt_num = 0;
                } else {
                    next_pkt_num += 1;
                }
                if n != BUFFER_SIZE {
                    return Err(io::Error::new(io::ErrorKind::Other, "buffer is not equal to n"));
                };
                //break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //println!("!!WouldBlock");
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }

        match socket_result.try_recv_from(&mut buf) {
            Ok((len_result, addr)) => {
                println!("socket_result len_result: {}, addr {:?}", len_result, addr);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //println!("socket_result WouldBlock");
            }
            Err(e) => {
                eprintln!("Error receiving data: {:?}", e);
            }
        }
    }
    Ok(())
}