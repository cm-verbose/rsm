use std::error::Error;

pub struct Reporter {}

impl Reporter {
  /// Report an error
  pub fn err<T: Error>(error: T) -> Result<(), String> {
    let message: String = error.to_string();
    Err(format!("Failed: {}", message))
  }
}
