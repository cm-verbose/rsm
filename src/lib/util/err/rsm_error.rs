use std::{
  error::Error,
  fmt::{Display, Formatter, Result},
  io,
};
use strum_macros::EnumIter;

/// Represents errors that can occur during the processing of ressources.
/// These errors try to abstract internal errors in a more manageable way.
#[derive(Debug, EnumIter)]
pub enum RSMError {
  DecompressionError,
  InvalidContent,
  InvalidFile,
  InvalidLength,
  NotEnoughContent,
  OutOfBounds,
  Other(String),
}

impl Display for RSMError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let message: &str = match self {
      Self::DecompressionError => "Failed to decompress content",
      Self::InvalidContent => "File contents are invalid",
      Self::InvalidFile => "Invalid file data or path",
      Self::InvalidLength => "Invalid lenght for data provided",
      Self::NotEnoughContent => "Not enough content to read",
      Self::OutOfBounds => "Value is out of bounds",
      Self::Other(msg) => &msg.to_string(),
    };
    write!(f, "{message}")
  }
}

impl Error for RSMError {}

impl From<io::Error> for RSMError {
  fn from(value: io::Error) -> Self {
    Self::Other(value.to_string())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use proptest::{prop_assert, prop_assert_eq, proptest};
  use std::{io::ErrorKind, mem::discriminant};
  use strum::IntoEnumIterator;

  /// Test the messages defined within [RSMError] are not empty for a way to
  /// identify causes when an error occurs. This excludes the
  /// [`RSMError::Other`] variant.
  #[test]
  fn test_errors_defined_types() {
    for error in RSMError::iter() {
      if discriminant(&error) == discriminant(&RSMError::Other("".to_string())) {
        continue;
      }
      assert!(!error.to_string().is_empty())
    }
  }

  proptest! {
    /// Test the [RSMError::Other](`RSMError::Other`) variant can appropriately
    /// generate error messages given a sequence of strings based on
    /// [`io::Error`].
    #[test]
    fn test_errors_other_mapping(message in ".+") {
      let error: io::Error = io::Error::new(ErrorKind::Other, message.clone());
      let mapped_error = RSMError::from(error);

      if let RSMError::Other(ref inner) = mapped_error {
        prop_assert_eq!(inner, &message);
        prop_assert!(!mapped_error.to_string().is_empty())
      } else {
        prop_assert!(false, "Expected a valid value v from RSMError::Other(v)");
      }
    }
  }
}
