use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;

use crate::structtun::Tun;

pub struct Client {
    stream: TcpStream,
    server_addr: String,
    device: Tun::Device,
}

impl Client {
    fn get_server_addr() -> String {
        // TODO: const addr only temp
        "10.0.0.9:2006".to_string()
    }

    #[cfg(target_os = "linux")]
    fn route() -> Result<(), io::Error> {
        // link
        {
            let args = ["link", "set", "up", "dev", "tun0"];
            let _link = Command::new("ip")
                .args(args)
                .output()
                .expect("Failed to do link");
        }
        // route
        {
            let args = [
                "route",
                "add",
                "0.0.0.0/0",
                "via",
                "10.0.0.9",
                "dev",
                "tun0",
            ];

            let _route = Command::new("ip")
                .args(args)
                .output()
                .expect("Failed to do route");
        }

        Ok(())
    }

    fn write_tun(&mut self) {
        let mut buf = [0u8, 2048];

        loop {
            match self.stream.read(&mut buf) {
                Ok(n) => {
                    // handle some shit with crypto. Decrypt and encrypt data
                    self.device.write(&buf).unwrap();
                }
                Err(e) => {
                    println!("Error reading: {}", e);
                    continue;
                }
            }
        }
    }

    pub fn init(&mut self) {
        self.server_addr = Self::get_server_addr();
        self.stream = TcpStream::connect(Self::get_server_addr()).unwrap();

        match tun::create(&Tun::init().unwrap()) {
            Ok(dev) => {
                self.device = dev;
            }
            Err(err) => {
                println!("{}", err)
            }
        }

        //routing
        if let Err(e) = Self::route() {
            eprintln!("Failed to set up TUN interface: {}", e);
            return;
        }

        let mut buf = [0u8; 1024];
        loop {
            match self.stream.read(&mut buf) {
                Ok(_) => {
                    // implement read from client and write to tun
                    Self::write_tun(self);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}
