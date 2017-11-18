
/// A SIRD record, based from a byte slice.
pub struct SIRD<'a> {
    count: u32,
    data: &'a [u8],
}

/// A single record into a SIRD file.
pub struct Record<'a> {
    name: &'str,
    data: &'a [u8],
}

/// The error type for decoding SIRD files.
pub enum SIRDError {
    /// The magic number was not correct.
    BadMagicNumber,

    /// A record was larger than expected.
    NotEnoughBytes,

    /// A record name contained invalid UTF-8.
    StringDecodeError,
}

/// The magic number expected to appear at the
/// start of all SIRD files.
pub const MAGIC: u32 = 0x53495244;
