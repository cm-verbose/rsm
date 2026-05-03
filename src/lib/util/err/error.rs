use crate::lib::util::err::png_error::PNGError;
use std::{
  error::Error,
  fmt::{Display, Formatter, Result},
  io,
};

/// An error occuring in [rsm](crate) by operations that process data.
#[derive(Debug)]
pub enum RSMError {
  /// PNG-specific errors.
  PNGError(PNGError),

  /// A default error used to cover undefined errors or non-[crate]-specific
  /// errors that may occur within the program.
  Other(String),
}

impl Display for RSMError {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
    match self {
      Self::PNGError(err) => err.fmt(fmt),
      Self::Other(message) => fmt.write_str(message),
    }
  }
}

impl Error for RSMError {}

impl From<io::Error> for RSMError {
  fn from(value: io::Error) -> Self {
    Self::Other(value.to_string())
  }
}
