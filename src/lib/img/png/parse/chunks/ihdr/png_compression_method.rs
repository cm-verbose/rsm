use crate::lib::util::err::rsm_error::RSMError;

/// Method in which the image was compressed
#[derive(Debug)]
pub enum CompressionMethod {
  Deflate = 0,
}

impl TryFrom<u8> for CompressionMethod {
  type Error = RSMError;

  fn try_from(method: u8) -> Result<Self, Self::Error> {
    match method {
      0 => Ok(Self::Deflate),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
