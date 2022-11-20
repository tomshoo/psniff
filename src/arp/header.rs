use crate::traits::Parsible;
use derive_builder::Builder;

use crate::{error::Error, ethertype::EtherType};

#[allow(unused)]
#[derive(Builder, Debug)]
pub struct ArpHeader<'a> {
    hwtype: u16,
    protocol: EtherType,
    hwsize: u8,
    prsize: u8,
    opcode: u16,
    snd_hwaddr: &'a [u8],
    snd_praddr: &'a [u8],
    rcv_hwaddr: &'a [u8],
    rcv_praddr: &'a [u8],
}

impl From<ArpHeaderBuilderError> for crate::error::Error {
    fn from(err: ArpHeaderBuilderError) -> Self {
        Error::new(crate::error::ErrorKind::General, Some(err.to_string()))
    }
}

#[allow(unused)]
impl<'a, 'b: 'a> TryFrom<&'b [u8]> for ArpHeader<'a> {
    type Error = crate::error::Error;
    fn try_from(slice: &'a [u8]) -> Result<Self, Self::Error> {
        let mut builder = ArpHeaderBuilder::<'a>::default();
        builder
            .hwtype(u16::from_be_bytes([slice[0], slice[1]]))
            .protocol(u16::from_be_bytes([slice[2], slice[3]]).try_into()?)
            .hwsize(slice[4])
            .prsize(slice[5])
            .opcode(u16::from_be_bytes([slice[6], slice[7]]));

        let hlen = slice[4] as usize;
        let plen = slice[5] as usize;

        let offset = hlen + plen;

        let addrslice = &slice[8..][..2 * offset];

        builder
            .snd_hwaddr(&addrslice[..hlen])
            .snd_praddr(&addrslice[hlen..offset])
            .rcv_hwaddr(&addrslice[offset..offset + hlen])
            .rcv_praddr(&addrslice[offset + hlen..]);

        Ok(builder.build()?)
    }
}

#[allow(dead_code)]
impl<'a> ArpHeader<'a> {
    pub fn from_slice<'b: 'a>(slice: &'b [u8]) -> crate::Result<(ArpHeader<'a>, &'b [u8])> {
        let header = Self::try_from(slice)?;
        let header_size = (header.hwsize + header.prsize) as usize * 2 + 8;

        Ok((header, &slice[header_size..]))
    }
}

impl<'a, 'p: 'a> Parsible<'p> for ArpHeader<'a> {
    type Error = Error;
    fn parse(slice: &'p [u8]) -> crate::Result<(Self, &[u8])>
    where
        Self: TryFrom<&'p [u8]>,
        'p: 'a,
    {
        Self::from_slice(slice)
    }
}
