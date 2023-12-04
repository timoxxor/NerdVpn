use crate::structtun::Tun;
use std::{error::Error, net::UdpSocket};

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
                let buf = pkt.get_bytes();
                let rec_msg = String::from_utf8(buf[..len].to_vec());

                println!("{:?}", rec_msg);
            }
            Err(err) => panic!("Error: {:?}", err),
        }
    }

    Ok(())
}
