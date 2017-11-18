extern crate byteorder;

/// The error type for decoding SIRD files.
#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    /// The magic number was not correct.
    BadMagicNumber,

    /// A record was larger than expected.
    NotEnoughBytes,

    /// A record name contained invalid UTF-8.
    StringDecodeError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;
        match *self {
            BadMagicNumber => write!(f, "incorrect magic number in SIRD file"),
            NotEnoughBytes => write!(f, "not enough bytes for record"),
            StringDecodeError => write!(f, "error decoding utf-8 string"),
        }
    }
}

/// A specialized `Result` type for SIRD operations.
pub type Result<T> = std::result::Result<T, Error>;

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

/// The magic number expected to appear at the
/// start of all SIRD files.
pub const MAGIC: u32 = 0x53495244;

impl<'a> SIRD<'a> {

    /// Create SIRD from byte slice. Fails if the magic number
    /// is invalid, or if there are not enough bytes to read
    /// the magic number etc.
    pub fn from_bytes(bs: &'a [u8]) -> Result<Self> {
        use byteorder::{ByteOrder, BE};

        if bs.len() < 8 {
            return Err(Error::NotEnoughBytes)
        }

        let magic = BE::read_u32(bs);
        if magic != MAGIC {
            return Err(Error::BadMagicNumber)
        }

        let count = BE::read_u32(&bs[4..]);
        Ok(SIRD { count, data: &bs[8..] })
    }

    /// Returns the number of records that are supposedly in this SIRD
    pub fn len(&self) -> usize {
        self.count as usize
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static R0_0: [u8; 8] = [0x53, 0x49, 0x52, 0x44, 0x00, 0x00, 0x00, 0x00];
    static R0_1: [u8; 8] = [0x53, 0x49, 0x52, 0x43, 0x00, 0x00, 0x00, 0x00];
    static R0_2: [u8; 6] = [0x53, 0x49, 0x52, 0x44, 0x00, 0x00];
    static R0_3: [u8; 10] = [0x53, 0x49, 0x52, 0x44, 0x00, 0x00, 0x01, 0x04, 0x7, 0x9];

    #[test]
    fn test_magic() {
        assert_eq!(SIRD::from_bytes(&R0_0).ok().map(|d| d.count), Some(0));
        assert_eq!(SIRD::from_bytes(&R0_1).err(), Some(Error::BadMagicNumber));
    }

    #[test]
    fn test_data_len() {
        assert_eq!(SIRD::from_bytes(&R0_2).err(), Some(Error::NotEnoughBytes));
        assert_eq!(SIRD::from_bytes(&R0_3).ok().map(|d| d.count), Some(260));
        assert_eq!(SIRD::from_bytes(&R0_3).ok().map(|d| d.len()), Some(260));
        assert_eq!(SIRD::from_bytes(&R0_3).ok().map(|d| d.data.len()), Some(2));
        assert_eq!(SIRD::from_bytes(&R0_3).ok().map(|d| d.data[0]), Some(7));
    }
}
