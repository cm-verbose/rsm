use std::{
  error::Error,
  fmt::{Display, Formatter, Result},
  io,
};

/// An error occuring in [rsm](crate) by operations that process data.
#[derive(Debug)]
pub enum RSMError {
  /// A default error used to cover undefined errors or non-[crate]-specific
  /// errors that may occur within the program.
  Other(String),
}

impl Display for RSMError {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
    let data: &str = match self {
      Self::Other(message) => message,
    };
    fmt.write_str(data)
  }
}

impl Error for RSMError {}

impl From<io::Error> for RSMError {
  fn from(value: io::Error) -> Self {
    Self::Other(value.to_string())
  }
}
