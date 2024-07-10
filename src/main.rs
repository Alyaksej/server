mod server;
mod test_client;

use std::{fs, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    server::run_server();
    test_client::run_client();
    Ok(())
}
