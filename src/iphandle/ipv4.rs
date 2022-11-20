use super::ipv4header;

pub fn handle(slice: &[u8]) -> crate::Result<()> {
    let (header, residue) = ipv4header::Ipv4Header::from_slice(slice)?;
    println!("{:?}", header);
    crate::print_as_ascii(residue);
    Ok(())
}
