use std::{error::Error, fmt::Display};

/// rsm errors
#[derive(Debug, PartialEq)]
pub enum RSMError {
  Empty,
  InvalidSignature,
  NotEnoughContent,
  OtherError(String),
}

impl Display for RSMError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Empty => write!(f, "No content provided"),
      Self::NotEnoughContent => write!(f, "Expected more content"),
      Self::InvalidSignature => write!(f, "Invalid image signature"),
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
