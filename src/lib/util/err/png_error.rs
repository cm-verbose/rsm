use std::fmt::{Display, Formatter, Result};

/// PNG-specific errors, whether in coding or decoding.
#[derive(Debug)]
pub enum PNGError {
  /// Out of bounds access to data within a PNG.
  OutOfBounds,

  /// Invalid prelude (Signature or IHDR)
  InvalidPrelude,
}

impl Display for PNGError {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
    let data = match self {
      Self::OutOfBounds => "Attempted to access data that is out of bounds",
      Self::InvalidPrelude => "Invalid signature or IHDR chunk header",
    };
    fmt.write_str(data)
  }
}
