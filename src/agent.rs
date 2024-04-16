use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use pcap::Capture;

use crate::config::Config;
use crate::core::poller::Poller;
use crate::packet_handler::PacketHandler;

#[derive(Clone)]
pub struct Agent {
    config: Config,
    running: Arc<AtomicBool>,
}

impl Agent {
    pub fn new(config: Config, running: Arc<AtomicBool>) -> Self {
        Self { 
            config,
            running,
        }
    }

    pub fn run(&self) -> u64 {
        let capture = Capture::from_device(self.config.get_device_name());
        if capture.is_err() {
            log::error!("Couldn't open a capture handle for a device: {}\nProvide a valid device", capture.err().unwrap());
            return 0;
        };
        let capture = capture.unwrap();
        let capture = capture.buffer_size(self.config.get_buffer_size()).open();
        if capture.is_err() {
            log::error!("Couldn't activates an inactive capture: {}", capture.err().unwrap());
            return 0;
        };

        let poller = Poller::builder()
            .with_capture(capture.unwrap())
            .with_handler(PacketHandler {
                directory: self.config.get_output_directory().to_string(),
            })
            .with_running(self.running.clone());
        let poller = match self.config.get_number_packages() {
            Some(packet_cnt) => poller.with_packet_cnt(packet_cnt),
            _ => poller,
        };
        poller.build().poll()
    }
}

impl Drop for Agent {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
}