use std::error::Error;

use futures::StreamExt;

struct Tun {}

impl Tun {
    pub fn init() -> tun::Configuration {
        let mut config = tun::Configuration::default();
        config
            .address((10, 0, 0, 9))
            .broadcast((192, 168, 102, 127))
            .netmask((255, 255, 255, 0))
            .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true);
        });

        config
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Tun::init();

    let dev = tun::create_as_async(&config).unwrap();
    let mut stream = dev.into_framed();

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
