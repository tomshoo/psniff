use crate::error::Error;

pub trait Parsible<'p>
where
  Error: From<<Self as Parsible<'p>>::Error>,
  Self: Sized,
{
  type Error;

  fn parse(slice: &'p [u8]) -> crate::Result<(Self, &[u8])>
  where
    Self: TryFrom<&'p [u8], Error = <Self as Parsible<'p>>::Error>;
}
