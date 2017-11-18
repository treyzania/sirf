extern crate byteorder;

/// A SIRD record, based from a byte slice.
pub struct SIRD<'a> {
    count: u32,
    data: &'a [u8],
}

/// A single record into a SIRD file.
pub struct Record<'a> {
    name: &'a str,
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

impl std::fmt::Display for SIRDError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::SIRDError::*;
        match *self {
            BadMagicNumber => write!(f, "incorrect magic number in SIRD file"),
            NotEnoughBytes => write!(f, "not enough bytes for record"),
            StringDecodeError => write!(f, "error decoding utf-8 string"),
        }
    }
}

pub type Result<T> = std::result::Result<T, SIRDError>;

/// The magic number expected to appear at the
/// start of all SIRD files.
pub const MAGIC: u32 = 0x53495244;
