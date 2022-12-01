// use crate::ethertype::EtherType;
// use crate::iphandle::ipv4header::Ipv4Header;
// use crate::iphandle::ipv6header::Ipv6Header;
use crate::traits::Parsible;

pub fn parse<'a, P>(slice: &'a [u8]) -> crate::Result<(P, &[u8])>
where
  P: Parsible<'a> + TryFrom<&'a [u8], Error = <P as Parsible<'a>>::Error>,
  crate::error::Error: From<<P as Parsible<'a>>::Error>,
{
  P::parse(slice)
}

// pub fn ether_handle(packet: &[u8], etype: Option<EtherType>) {
//     if let Some(etype) = etype {
//         match etype {
//             EtherType::Ipv4 => println!("{:?}", parse::<Ipv4Header>(packet).unwrap()),
//             EtherType::Ipv6 => println!("{:?}", parse::<Ipv6Header>(packet).unwrap()),
//             _ => {}
//         }
//     }
//
//     crate::raw_packet(packet).unwrap_or(())
// }
