pub mod header;

use crate::print_as_ascii;

pub use header::ArpHeader;

pub fn handle(slice: &[u8]) -> crate::Result<()> {
  let (header, residue) = ArpHeader::from_slice(slice)?;
  println!("{:?}", header);
  print_as_ascii(residue);
  Ok(())
}
