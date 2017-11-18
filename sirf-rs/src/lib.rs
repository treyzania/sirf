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

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Error::StringDecodeError
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

pub struct RecordIterator<'a> {
    data: &'a [u8],
    remaining: usize,
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

    /// Returns the number of records that are supposedly in this SIRD.
    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Returns an iterator over the records in this SIRD.
    pub fn iter(&self) -> RecordIterator<'a> {
        RecordIterator {
            data: self.data,
            remaining: self.count as usize,
        }
    }
}

impl<'a> RecordIterator<'a> {

    /// Try to read a record
    fn try_next(&mut self) -> Result<Record<'a>> {
        use byteorder::{ByteOrder, BE};

        if self.data.len() < 11 {
            return Err(Error::NotEnoughBytes)
        }

        let size = BE::read_u64(self.data) as usize;
        let name_len = BE::read_u16(&self.data[8..]) as usize;
        let name_bs = &self.data[10..(10 + name_len)];

        if self.data.len() < 11 + name_len + size {
            return Err(Error::NotEnoughBytes)
        }

        let name = std::str::from_utf8(name_bs)?;
        let data = &self.data[(11 + name_len)..(11 + name_len + size)];
        self.data = &self.data[(11 + name_len + size)..];
        Ok(Record { name, data })
    }
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = Result<Record<'a>>;

    fn next(&mut self) -> Option<Result<Record<'a>>> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.try_next()
                 .map_err(|e| {
                     // kill the iterator if an error occurs
                     self.remaining = 0;
                     e
                 }))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static R0_0: [u8; 8] = [0x53, 0x49, 0x52, 0x44, 0x00, 0x00, 0x00, 0x00];
    static R0_1: [u8; 8] = [0x53, 0x49, 0x52, 0x43, 0x00, 0x00, 0x00, 0x00];
    static R0_2: [u8; 6] = [0x53, 0x49, 0x52, 0x44, 0x00, 0x00];
    static R0_3: [u8; 10] = [0x53, 0x49, 0x52, 0x44, 0x00, 0x00, 0x01, 0x04, 0x7, 0x9];

    static R1: [u8; 25] = [0x53, 0x49, 0x52, 0x44,
                           0x00, 0x00, 0x00, 0x01, /* 1 entry */
                           0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x03,
                           /* "foo" */ 0x66, 0x6f, 0x6f, 0x00,
                           /* "bar" */ 0x62, 0x61, 0x72];

    static R2: [u8; 39] = [0x53, 0x49, 0x52, 0x44,
                           0x00, 0x00, 0x00, 0x02, /* 2 entries */
                           0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
                           /* "hi" */ 0x68, 0x69, 0x00,
                           123,
                           0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04,
                           /* "sird" */ 0x73, 0x69, 0x72, 0x64, 0x00,
                           45, 67];

    static R3: [u8; 20] = [0x53, 0x49, 0x52, 0x44,
                           0x00, 0x00, 0x00, 0x06, /* 6 entries */
                           0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
                           /* "hi" */ 0x68, 0x69];


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

    #[test]
    fn test_empty() {
        let sird = SIRD::from_bytes(&R0_0).unwrap();
        assert_eq!(sird.iter().count(), 0);
    }

    #[test]
    fn test_foo_bar() {
        let sird = SIRD::from_bytes(&R1).unwrap();
        {
            let mut it = sird.iter();
            let rec0 = it.next().unwrap().unwrap();
            assert_eq!(rec0.name, "foo");
            assert_eq!(rec0.data, &[0x62, 0x61, 0x72]);
            assert_eq!(it.remaining, 0);
            assert_eq!(it.data.len(), 0);
            assert_eq!(it.next().is_none(), true);
        }
    }

    #[test]
    fn test_hi_sird() {
        let sird = SIRD::from_bytes(&R2).unwrap();
        let recs: Vec<_> = sird.iter().map(|r| r.unwrap()).collect();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].name, "hi");
        assert_eq!(recs[0].data, &[123]);
        assert_eq!(recs[1].name, "sird");
        assert_eq!(recs[1].data, &[45, 67]);
    }

    #[test]
    fn test_malform() {
        let sird = SIRD::from_bytes(&R3).unwrap();
        let ress: Vec<_> = sird.iter().collect();
        assert_eq!(ress.len(), 1);
        assert_eq!(&ress[0].as_ref().err(), &Some(&Error::NotEnoughBytes));
    }
}
