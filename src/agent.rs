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
        let capture = Capture::from_device(self.config.get_device_name())
            .unwrap()
            .buffer_size(self.config.get_buffer_size())
            .open()
            .unwrap();

        Poller::new(capture)
            .with_packet_cnt(self.config.get_number_packages())
            .with_codec(PacketHandler {
                directory: self.config.get_output_directory().to_string(),
            })
            .poll();
    }
}