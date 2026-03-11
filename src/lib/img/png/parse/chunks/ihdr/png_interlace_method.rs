use crate::lib::util::err::rsm_error::RSMError;

/// Interlace method
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InterlaceMethod {
  Null = 0,
  Adam7 = 1,
}

impl TryFrom<u8> for InterlaceMethod {
  type Error = RSMError;

  fn try_from(method: u8) -> Result<Self, Self::Error> {
    match method {
      0 => Ok(Self::Null),
      1 => Ok(Self::Adam7),
      _ => Err(RSMError::InvalidContent),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::img::png::parse::chunks::ihdr::png_interlace_method::InterlaceMethod;
  use proptest::prelude::*;

  /// Test mapping to existing interlace methods
  #[test]
  fn test_interlace_mapping() {
    type Method = InterlaceMethod;
    let methods: [Method; 2] = [Method::Null, Method::Adam7];

    for (num, expected) in (0..=methods.len() as u8).zip(methods) {
      let method: Method = num.try_into().unwrap();
      assert_eq!(method, expected)
    }
  }

  proptest! {
    /// Test invalid methods
    #[test]
    fn test_invalid_values(method in 2..=u8::MAX) {
      let interlace_method: Result<InterlaceMethod, _> = method.try_into();
      assert!(interlace_method.is_err())
    }
  }
}
