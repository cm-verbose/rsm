use crate::lib::util::err::rsm_error::RSMError;

/// Method in which the image was filtered
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FilterMethod {
  Method0 = 0,
}

impl TryFrom<u8> for FilterMethod {
  type Error = RSMError;

  fn try_from(method: u8) -> Result<Self, Self::Error> {
    match method {
      0 => Ok(Self::Method0),
      _ => Err(RSMError::InvalidContent),
    }
  }
}

#[cfg(test)]
pub mod tests {
  use crate::lib::img::png::parse::chunks::ihdr::png_filter_method::FilterMethod;
  use proptest::prelude::*;

  /// Test mapping 0 to Method0
  #[test]
  fn test_filter_mapping_method0() {
    let method: u8 = 0;
    let filter_method: FilterMethod = method.try_into().unwrap();
    assert_eq!(filter_method, FilterMethod::Method0);
  }

  proptest! {
    /// Test invalid methods
    #[test]
    fn test_invalid_values(method in 1..=u8::MAX) {
      let filter_method: Result<FilterMethod, _> = method.try_into();
      assert!(filter_method.is_err())
    }
  }
}
