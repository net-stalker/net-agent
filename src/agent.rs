use pcap::Capture;

use crate::config::Config;
use crate::core::poller::Poller;
use crate::packet_handler::PacketHandler;

pub struct Agent {
    config: Config,
}

impl Agent {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(self) {
        let capture = Capture::from_device(self.config.capture.device_name.as_str())
            .unwrap()
            .buffer_size(self.config.capture.buffer_size)
            .open()
            .unwrap();

        Poller::new(capture)
            .with_packet_cnt(self.config.capture.number_packages)
            .with_codec(PacketHandler {
                directory: "output".to_string(),
            })
            .poll();
    }
}