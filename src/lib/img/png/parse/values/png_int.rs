use std::ops::Deref;

use crate::lib::util::err::rsm_error::RSMError;

/// PNG 4 bytes unsigned integer
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PNGInt(pub u32);

impl TryFrom<[u8; 4]> for PNGInt {
  type Error = RSMError;

  fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
    let num: u32 = u32::from_be_bytes(bytes);
    if num > i32::MAX as u32 {
      Err(RSMError::OutOfBounds)
    } else {
      Ok(Self(num))
    }
  }
}

impl TryFrom<&[u8]> for PNGInt {
  type Error = RSMError;

  fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
    let value: [u8; 4] = bytes.try_into().map_err(|_| RSMError::InvalidContent)?;
    Self::try_from(value)
  }
}

impl From<PNGInt> for u32 {
  fn from(png_int: PNGInt) -> Self {
    png_int.0
  }
}

impl Deref for PNGInt {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
