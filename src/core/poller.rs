use std::{error::Error, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use net_file::translator::packet::Packet;
use pcap::{Active, Capture};

pub struct Poller<H> {
    capture: Capture<Active>,
    packet_cnt: Option<u64>,
    handler: H,
    running: Arc<AtomicBool>,
}

pub trait Handler {
    fn decode(&self, packet: Packet) -> Result<(), Box<dyn Error + Send + Sync>>;
}

impl<H: Handler> Poller<H> {
    fn new(capture: Capture<Active>, packet_cnt: Option<u64>, handler: H, running: Arc<AtomicBool>) -> Self {
        Poller {
            capture,
            packet_cnt,
            handler,
            running,
        }
    }

    pub fn builder() -> PollerBuilder<H> {
        PollerBuilder::default()
    }

    pub fn poll(&mut self) -> u64 {
        let mut cnt = 0_u64;

        while self.running.load(Ordering::SeqCst) && cnt < u64::MAX && ((self.packet_cnt.is_some() && cnt != *self.packet_cnt.as_ref().unwrap()) || self.packet_cnt.is_none()) {
            let packet = self.capture.next_packet();
            let packet = match packet {
                Ok(packet) => packet,
                Err(err) => {
                    log::error!("Something went wrong during capturing packets: {}", err);
                    return cnt;
                }
            };
            match self.handler.decode(Packet::from(packet)) {
                Ok(_) => (),
                Err(err) => {
                    log::error!("{err}");
                    return cnt;
                }
            };
            cnt += 1;
        }
        cnt
    }
}


pub struct PollerBuilder<H: Handler> {
    capture: Option<Capture<Active>>,
    packet_cnt: Option<u64>,
    handler: Option<H>,
    running: Option<Arc<AtomicBool>>,
}

impl<H: Handler> Default for PollerBuilder<H> {
    fn default() -> Self {
        Self {
            capture: None,
            packet_cnt: None,
            handler: None,
            running: None,
        }
    }
}


impl<H: Handler> PollerBuilder<H> {
    pub fn with_packet_cnt(mut self, packet_cnt: u64) -> Self {
        self.packet_cnt = Some(packet_cnt);
        self
    }

    pub fn with_handler(mut self, handler: H) -> Self {
        self.handler = Some(handler);
        self
    }

    pub fn with_running(mut self, running: Arc<AtomicBool>) -> Self {
        self.running = Some(running);
        self
    }

    pub fn with_capture(mut self, capture: Capture<Active>) -> Self {
        self.capture = Some(capture);
        self
    }

    pub fn build(self) -> Poller<H> {
        Poller::new(
            self.capture.unwrap(),
            self.packet_cnt,
            self.handler.unwrap(),
            self.running.unwrap(),
        )
    }
}
