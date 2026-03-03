use crate::lib::{img::png::read::reader::png_reader::PNGReader, util::rsm_error::RSMError};

impl<'a> PNGReader<'a> {
  /// PNG image signature
  const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa];

  /// Validate the PNG image's signature
  pub(super) fn validate_signature(&self, bytes: &'a [u8]) -> Result<(), RSMError> {
    if bytes.len() < 8 {
      return Err(RSMError::NotEnoughContent);
    }
    if bytes.get(0..8) == Some(&Self::SIGNATURE) {
      Ok(())
    } else {
      Err(RSMError::InvalidSignature)
    }
  }
}

#[cfg(test)]
pub mod tests {
  use super::PNGReader;
  use proptest::{
    prelude::{any, prop},
    prop_assert, proptest,
  };

  /// Test valid signatures
  #[test]
  fn validate_valid_png_signature() {
    let reader: PNGReader = PNGReader::new();
    let expected: bool = reader
      .validate_signature(&[0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa])
      .is_ok();
    assert!(expected);
  }

  proptest! {
    /// Test invalid signatures
    #[test]
    fn invalidate_invalid_png_signatures(bytes in prop::collection::vec(any::<u8>(), 0..12)) {
      let reader: PNGReader = PNGReader::new();
      let result = reader.validate_signature(&bytes);

      if bytes.len() == 8 && &bytes[0..8] == &PNGReader::SIGNATURE {
        prop_assert!(result.is_ok())
      } else {
        prop_assert!(result.is_err())
      }
    }
  }
}
