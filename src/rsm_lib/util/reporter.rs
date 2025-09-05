use std::{io::Error, path::Path};

pub struct Reporter;

impl Reporter {
  pub fn report_missing_file<T>(target_path: &Path) -> Result<T, String> {
    let message: String = format!("Failed fiding path: {}", target_path.display());
    Err(message)
  }

  pub fn report_dir_instead_of_file<T>(target_path: &Path) -> Result<T, String> {
    let message: String = format!(
      "Found a directory instead of a path at {}",
      target_path.display()
    );
    Err(message)
  }

  pub fn report_failed_reading_file<T>(target_path: &Path, error: Error) -> Result<T, String> {
    let message: String = format!("Failed reading \"{}\": {}", target_path.display(), error);
    Err(message)
  }

  pub fn report_under_minimum_bytes<T>(bytes: usize) -> Result<T, String> {
    let message: String = format!(
      "Failed to reach the minimum amount of bytes ({}) for this data to be valid",
      bytes
    );
    Err(message)
  }
}
