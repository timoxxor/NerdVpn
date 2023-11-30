use crate::structtun::Tun;
use std::error::Error;

use futures::StreamExt;

mod structtun;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream;

    match Tun::init() {
        Some(config) => {
            stream = tun::create_as_async(&config).unwrap().into_framed();
        }
        None => panic!("Can't configure tun"),
    }

    while let Some(packet) = stream.next().await {
        match packet {
            Ok(pkt) => {
                // do some crazy shit with networking
                let amount = pkt.get_bytes();
                let len = amount.len();
                println!("Size: {:?}", len);
            }
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    Ok(())
}
