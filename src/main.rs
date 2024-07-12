mod server;
mod test_client;

use std::{io};
use std::env;
use std::io::Error;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("!!!!!!!!!!!!!!!!!!!!!{:?}", args[1]);

    if args.len() > 1 && args[1] == "mode=server" {
        let _ =server::run_server().await;
    } else if args.len() > 1 && args[1] == "mode=test_client" {
        let _ = test_client::run_test_client().await;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "ERROR: Invalid mode"));
    }

    Ok(())
}
