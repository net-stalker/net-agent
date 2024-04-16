use net_file::translator::{global_header::GlobalHeader, packet::Packet};
use std::fs::File;
use uuid::Uuid;
use std::io::prelude::*;

fn write_to_file(buf: &[u8], directory: &str) -> std::io::Result<()> {
    let file_name = format!("{}/{}.pcap", directory, Uuid::new_v4());
    let mut file = File::create(file_name)?;
    file.write_all(buf)?;
    Ok(())
}

pub struct PacketHandler { 
    pub directory: String,
}

impl crate::core::poller::Handler for PacketHandler {
    fn decode(&self, packet: Packet) {
        let global_header = GlobalHeader::default();

        let mut buf = global_header.to_bytes();
        buf.append(&mut packet.to_bytes());

        match write_to_file(&buf, &self.directory) {
            Err(err) => log::error!("Failed to write to file: {}", err),
            Ok(_) => log::debug!("Packet written to file successfully!")
        };
    }
}
