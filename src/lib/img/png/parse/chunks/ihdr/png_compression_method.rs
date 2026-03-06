use crate::lib::util::err::rsm_error::RSMError;

/// Method in which the image was compressed
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
  use crate::lib::img::png::parse::chunks::ihdr::png_compression_method::CompressionMethod;
  use proptest::prelude::*;

  #[test]
  fn test_compression_mapping() {
    let method: CompressionMethod = 0.try_into().unwrap();
    assert_eq!(method, CompressionMethod::Deflate);
  }

  proptest! {
    /// Test invalid methods
    #[test]
    fn test_invalid_values(method in 1..=u8::MAX) {
      let compression_method: Result<CompressionMethod, _> = method.try_into();
      assert!(compression_method.is_err())
    }
  }
}
