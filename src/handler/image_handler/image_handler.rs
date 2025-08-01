use std::{fs, path::Path};
use crate::handler::image_handler::handlers::png::PNGHandler;

// Handles reading images and returns data
pub struct ImageHandler;

impl ImageHandler {
  /// Read an image content from its path
  pub fn read_from_path(&self, path: &Path) -> Result<(), String> {
    if !path.exists() {
      return Err(format!("File at \"{:?}\" does not exist.", path));
    }

    if let Some(os_str) = path.extension() {
      if let Some(ext) = os_str.to_str() {
        let mut parser: Box<dyn ImageParser> = match ext {
          "png" => Ok(Box::new(PNGHandler::new())),
          _ => Err(format!("Unsupported format \"{}\"", ext)),
        }?;
        parser.parse(fs::read(path).unwrap())?;
      }
    }
    return Ok(());
  }
}

pub trait ImageParser {
  fn new() -> Self
  where
    Self: Sized;

  /// Parse image contents into metadata
  fn parse(&mut self, contents: Vec<u8>) -> Result<(), String>;
}
