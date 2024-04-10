use net_file::translator::packet::Packet;
use pcap::{Active, Capture};

pub struct Poller<H> {
    capture: Capture<Active>,
    packet_cnt: i32,
    handler: Option<H>,
    capturing_payload: bool,
}

pub trait Handler {
    fn decode(&self, cnt: i32, packet: Packet);
}

impl<H: Handler> Poller<H> {
    pub fn new(capture: Capture<Active>) -> Self {
        let infinite_capturing = -1;

        Poller {
            capture,
            packet_cnt: infinite_capturing,
            handler: None,
            capturing_payload: false,

        }
    }

    pub fn with_packet_cnt(&mut self, packet_cnt: i32) -> &mut Self {
        self.packet_cnt = packet_cnt;
        self
    }

    pub fn with_codec(&mut self, handler: H) -> &mut Self {
        self.handler = Some(handler);
        self
    }

    pub fn with_payload_capture(&mut self, capturing_payload: bool) -> &mut Self {
        self.capturing_payload = capturing_payload;
        self
    }

    pub fn poll(&mut self) {
        let mut cnt = 0_i32;
        while let Ok(packet) = self.capture.next_packet() {
            if self.packet_cnt == cnt {
                break;
            }
            cnt += 1;
            
            match &self.handler {
                None => {}
                Some(handler) => {
                    let packet = Packet::from(packet);
                    handler.decode(cnt, packet);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn test() {
        let payload_capture = false;
        let capture = Capture::from_device("en0")
            .unwrap()
            // .promisc(true)
            // .snaplen(65535)
            .buffer_size(1000)
            .open()
            .unwrap();

        struct Codec;
        impl Handler for Codec {
            fn decode(&self, cnt: i32, packet: Packet) {
                println!("Received packet: cnt={} packet={:?}", cnt, packet);
            }
        }

        Poller::new(capture)
            .with_packet_cnt(1)
            .with_codec(Codec)
            .with_payload_capture(payload_capture)
            .poll();
    }
}