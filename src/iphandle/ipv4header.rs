use derive_builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Ipv4Header<'a> {
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

    srcaddr: &'a [u8],
    dstaddr: &'a [u8],

    options: &'a [u8],
}

impl From<Ipv4HeaderBuilderError> for crate::error::Error {
    fn from(err: Ipv4HeaderBuilderError) -> Self {
        Self::new(crate::error::ErrorKind::ParserError, Some(err.to_string()))
    }
}

impl<'a, 'b: 'a> TryFrom<&'b [u8]> for Ipv4Header<'a> {
    type Error = crate::error::Error;
    fn try_from(slice: &'b [u8]) -> Result<Self, Self::Error> {
        let mut builder = Ipv4HeaderBuilder::<'a>::create_empty();

        if slice[0] >> 4 != 4 {
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
            .ecn(slice[1] << 6 >> 6)
            .id(u16::from_be_bytes([slice[4], slice[5]]))
            .flags(slice[6] >> 5)
            .frag_offset(u16::from_be_bytes([slice[6] << 5 >> 5, slice[7]]))
            .ttl(slice[8])
            .protocol(slice[9])
            .checksum(u16::from_be_bytes([slice[10], slice[11]]))
            .srcaddr(&slice[12..16])
            .dstaddr(&slice[16..20])
            .options(&header_slice[20..])
            .build();

        Ok(header?)
    }
}
