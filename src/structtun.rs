pub struct Tun {}

impl Tun {
    pub fn init() -> Option<tun::Configuration> {
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

        Some(config)
    }
}
