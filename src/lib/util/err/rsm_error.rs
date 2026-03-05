use std::{error::Error, fmt::Display};

/// rsm errors
#[derive(Debug, PartialEq)]
pub enum RSMError {
  Empty,
  InvalidContent,
  InvalidLength,
  InvalidSignature,
  NotEnoughContent,
  OtherError(String),
}

impl Display for RSMError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Empty => write!(f, "No content provided"),
      Self::InvalidContent => write!(f, "Invalid content passed"),
      Self::InvalidLength => write!(f, "Invalid length read"),
      Self::InvalidSignature => write!(f, "Invalid image signature"),
      Self::NotEnoughContent => write!(f, "Expected more content"),
      Self::OtherError(msg) => write!(f, "{msg}"),
    }
  }
}

impl Error for RSMError {}

impl From<std::io::Error> for RSMError {
  fn from(value: std::io::Error) -> Self {
    RSMError::OtherError(value.to_string())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use proptest::prelude::{prop_assert, prop_assert_eq, proptest};
  use std::io::{self, ErrorKind};

  #[test]
  fn test_existing_error_types() {
    let error_types: [RSMError; 5] = [
      RSMError::Empty,
      RSMError::InvalidContent,
      RSMError::InvalidLength,
      RSMError::InvalidSignature,
      RSMError::NotEnoughContent,
    ];

    for error_type in error_types {
      assert!(!error_type.to_string().is_empty());
    }
  }

  proptest! {
    /// Tests io errors wrap correctly
    #[test]
    fn test_native_error_wrapping(message in ".+") {
      let error = io::Error::new(ErrorKind::Other, message.clone());
      let err = RSMError::from(error);

      if let RSMError::OtherError(ref inner_err) = err {
        prop_assert_eq!(inner_err, &message);
        prop_assert!(!err.to_string().is_empty())
      } else {
        prop_assert!(false, "Expected RSMError::OtherError(_)")
      }
    }
  }
}
