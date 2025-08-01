use crate::handler::image_handler::ImageHandler;
use std::path::Path;

/// Handle reading files and other types
pub struct Handler;

impl Handler {
  pub fn new() -> Self {
    Self {}
  }

  /// Read contents from a file with a given path
  pub fn read_from_path(&self, path: &Path) -> Result<(), String> {
    if !path.exists() {
      return Err(format!("File at \"{:?}\" does not exist.", path));
    }

    if let Some(os_str) = path.extension() {
      if let Some(ext) = os_str.to_str() {
        match ext {
          "png" => {
            let _: () = ImageHandler.read_from_path(path)?;
          }
          _ => {
            return Err(format!("Unsupported image format \"{}\"", ext));
          }
        }
      }
    }
    return Ok(());
  }
}
