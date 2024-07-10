mod server;
mod test_client;

use std::{io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let serv = server::run_server();
    let clie = test_client::run_test_client();
    Ok(())
}
