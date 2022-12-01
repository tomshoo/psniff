use derive_builder::Builder;

use crate::{error::Error, traits::Parsible};

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Ipv4Header {
  header_length: u8,
  packet_len: u16,
  dscp: u8,
  ecn: u8,
  id: u16,
  flags: u8,
  frag_offset: u16,
  ttl: u8,
  protocol: u8,

  checksum: u16,

  srcaddr: [u8; 4],
  dstaddr: [u8; 4],

  options: Vec<u8>,
}

impl From<Ipv4HeaderBuilderError> for crate::error::Error {
  fn from(err: Ipv4HeaderBuilderError) -> Self {
    Self::new(crate::error::ErrorKind::ParserError, Some(err.to_string()))
  }
}

impl TryFrom<&'_ [u8]> for Ipv4Header {
  type Error = Error;
  fn try_from(slice: &'_ [u8]) -> Result<Self, Self::Error> {
    let mut builder = Ipv4HeaderBuilder::create_empty();

    if slice[0] >> 4 != 4 || slice.len() < 20 {
      return Err(crate::error::Error::new(
        crate::error::ErrorKind::ParserError,
        Some(String::from("Not an Ipv4 Packet")),
      ));
    };

    let header_len = (slice[0] << 4 >> 4) * 4;
    let header_slice = &slice[..header_len as usize];

    let header = builder
      .header_length(header_len)
      .dscp(slice[1] >> 6)
      .packet_len(u16::from_be_bytes([slice[2], slice[3]]))
      .ecn(slice[1] & 0b111111)
      .id(u16::from_be_bytes([slice[4], slice[5]]))
      .flags(slice[6] >> 5)
      .frag_offset(u16::from_be_bytes([slice[6] & 0b11111, slice[7]]))
      .ttl(slice[8])
      .protocol(slice[9])
      .checksum(u16::from_be_bytes([slice[10], slice[11]]))
      .srcaddr(slice[12..16].try_into().unwrap())
      .dstaddr(slice[16..20].try_into().unwrap())
      .options(header_slice[20..].to_vec())
      .build()?;

    Ok(header)
  }
}

impl Ipv4Header {
  pub fn from_slice(slice: &'_ [u8]) -> crate::Result<(Ipv4Header, &'_ [u8])> {
    let ipheader = Ipv4Header::try_from(slice)?;
    let length = ipheader.header_length as usize;
    Ok((ipheader, &slice[length..]))
  }
}

impl<'p> Parsible<'p> for Ipv4Header {
  type Error = Error;
  fn parse(slice: &'p [u8]) -> crate::Result<(Self, &[u8])>
  where
    Self: TryFrom<&'p [u8]>,
  {
    Self::from_slice(slice)
  }
}
