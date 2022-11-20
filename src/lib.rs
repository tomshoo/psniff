pub mod arp;
pub mod error;
pub mod ethertype;
pub mod iphandle;
pub mod marker;
pub mod traits;

pub mod handlers;

use crate::ethertype::EtherType;
use crate::handlers::parse;
use etherparse::Ethernet2Header;
use std::io::Write;

pub type Result<T> = std::result::Result<T, error::Error>;

pub fn raw_packet(slice: &[u8]) -> Option<()> {
    match slice[0] >> 4 {
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
    println!("{:?}", packet.header);
    std::io::stdout()
        .lock()
        .write_all("-----------------------------------------------------\n".as_bytes())
        .unwrap();

    if let Ok((ethdr, etdata)) = Ethernet2Header::from_slice(packet.data) {
        let Ok(etype) = EtherType::try_from(ethdr.ether_type) else {
            if raw_packet(packet.data).is_none() {
                print_as_bytes(packet.data)
            }
            return;
        };

        println!("{:?}", ethdr);

        let packet = match etype {
            EtherType::Ipv4 => {
                dbg!(parse::<crate::iphandle::ipv4header::Ipv4Header>(etdata).unwrap()).1
            }
            EtherType::Ipv6 => {
                dbg!(parse::<crate::iphandle::ipv6header::Ipv6Header>(etdata).unwrap()).1
            }
            EtherType::Arp => dbg!(parse::<crate::arp::ArpHeader>(etdata).unwrap()).1,
        };

        print_as_ascii(packet)
    } else {
        print_as_bytes(packet.data)
    }
}
