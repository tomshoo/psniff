#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum EtherType {
  Ipv4,
  Ipv6,
  Arp,
}

impl TryFrom<u16> for EtherType {
  type Error = String;
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    match value {
      0x0800 => Ok(Self::Ipv4),
      0x86dd => Ok(Self::Ipv6),
      0x0806 => Ok(Self::Arp),
      _ => Err(format!("{} is not a valid ethertype", value)),
    }
  }
}
