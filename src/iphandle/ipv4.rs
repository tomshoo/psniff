use super::ipv4header;
use etherparse::Ipv4Header;

pub fn handle(slice: &[u8]) -> crate::Result<()> {
    let (header, residue) = Ipv4Header::from_slice(slice)?;
    println!("{:?}", header);
    crate::print_as_ascii(residue);
    let _ = ipv4header::Ipv4Header::try_from(slice);
    Ok(())
}
