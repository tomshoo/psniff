pub mod arp;
pub mod error;
pub mod ethertype;
pub mod iphandle;

use crate::ethertype::EtherType;
use etherparse::Ethernet2Header;
use std::io::Write;

pub type Result<T> = std::result::Result<T, error::Error>;

pub fn raw_packet(slice: &[u8]) -> Option<()> {
    match dbg!(slice[0] >> 4) {
        iphandle::IPV4 => iphandle::ipv4::handle(slice).ok(),
        iphandle::IPV6 => iphandle::ipv6::handle(slice).ok(),
        _ => None,
    }
}

fn print_as_bytes(packet: &[u8]) {
    println!("Hex: {:x?}", packet);
    println!("Dec: {:?}", packet);
}

fn print_as_ascii(slice: &[u8]) {
    slice.iter().for_each(|byte| match *byte as char {
        ch if ch.is_ascii_alphanumeric() | ch.is_ascii_punctuation() => print!("{}", ch),
        _ => print!("."),
    });
    println!();
}

#[allow(unreachable_patterns)]
pub fn analyze_packet(packet: pcap::Packet) {
    if matches!(raw_packet(packet.data), Some(..)) {
        return;
    }

    std::io::stdout()
        .lock()
        .write_all("-----------------------------------------------------\n".as_bytes())
        .unwrap();
    if let Ok((ethdr, etdata)) = Ethernet2Header::from_slice(packet.data) {
        println!("{:?}", ethdr);
        if let Ok(ethtype) = EtherType::try_from(ethdr.ether_type) {
            match ethtype {
                EtherType::Ipv4 => iphandle::ipv4::handle(etdata).unwrap(),
                EtherType::Ipv6 => iphandle::ipv6::handle(etdata).unwrap(),
                EtherType::Arp => arp::handle(etdata).unwrap(),
                // _ => print_as_bytes(etdata),
            }
        } else {
            print_as_bytes(etdata)
        }
    }

    // print_as_bytes(&packet);
    print_as_ascii(packet.data);
}
