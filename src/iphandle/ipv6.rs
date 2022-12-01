use super::ipv6header;

pub fn handle(slice: &[u8]) -> crate::Result<()> {
  let header = ipv6header::Ipv6Header::from_slice(slice)?;

  println!("{:?}", header);
  Ok(())
}
