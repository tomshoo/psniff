use crate::error::{Error, ErrorKind};
use crate::traits::Parsible;
use derive_builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Ipv6Header {
  traffic_class: u8,
  flow_label: [u8; 3],
  payload_length: u16,
  next_header: u8,
  hop_limit: u8,
  src_adrr: [u8; 16],
  dst_addr: [u8; 16],
}

impl From<Ipv6HeaderBuilderError> for crate::error::Error {
  fn from(err: Ipv6HeaderBuilderError) -> Self {
    Self::new(ErrorKind::ParserError, Some(err.to_string()))
  }
}

impl TryFrom<&'_ [u8]> for Ipv6Header {
  type Error = crate::error::Error;

  fn try_from(slice: &'_ [u8]) -> Result<Self, Self::Error> {
    let mut builder = Ipv6HeaderBuilder::create_empty();

    if slice[0] >> 4 != 6 || slice.len() < 40 {
      return Err(Error::new(
        ErrorKind::ParserError,
        Some(String::from("not a valid ipv6 packet")),
      ));
    }

    let traffic_class = (slice[0] << 4) | (slice[1] >> 4);
    builder
      .traffic_class(traffic_class)
      .flow_label([slice[1] & 0b1111, slice[2], slice[3]])
      .payload_length(u16::from_be_bytes([slice[4], slice[5]]))
      .next_header(slice[6])
      .hop_limit(slice[7])
      .src_adrr(slice[8..24].try_into().unwrap())
      .dst_addr(slice[24..40].try_into().unwrap());

    Ok(builder.build()?)
  }
}

#[allow(dead_code)]
impl Ipv6Header {
  pub fn ecn(&self) -> u8 {
    self.traffic_class & 0b11
  }

  pub fn dscp(&self) -> u8 {
    self.traffic_class >> 2
  }

  pub fn flow_label(&self) -> u32 {
    let mut bytes = [0; 4];
    for (x, y) in self.flow_label.iter().zip(bytes[1..].iter_mut()) {
      *y = *x
    }
    u32::from_be_bytes(bytes)
  }

  pub fn from_slice(slice: &'_ [u8]) -> crate::Result<(Ipv6Header, &'_ [u8])> {
    Ok((Self::try_from(slice)?, &slice[40..]))
  }
}

impl<'p> Parsible<'p> for Ipv6Header {
  type Error = Error;
  fn parse(slice: &'p [u8]) -> crate::Result<(Self, &[u8])>
  where
    Self: TryFrom<&'p [u8]>,
  {
    Self::from_slice(slice)
  }
}
