use super::{SIG_COMPRESSED, SIG_UNCOMPRESSED};
use crate::error::KbinError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compression {
    Compressed,
    Uncompressed,
}

impl Compression {
    pub fn from_byte(byte: u8) -> Result<Self, KbinError> {
        match byte {
            SIG_COMPRESSED => Ok(Compression::Compressed),
            SIG_UNCOMPRESSED => Ok(Compression::Uncompressed),
            _ => Err(KbinError::UnknownCompression),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match *self {
            Compression::Compressed => SIG_COMPRESSED,
            Compression::Uncompressed => SIG_UNCOMPRESSED,
        }
    }
}

impl Default for Compression {
    fn default() -> Self {
        Compression::Compressed
    }
}
