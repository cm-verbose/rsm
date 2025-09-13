use std::{fs, path::Path};

use crate::rsm_lib::img::png::{image::png_image::PNGImage, reader::png_reader::PNGReader};

/// Handle PNG files
pub struct PNGHandler;

impl PNGHandler {
  pub fn new() -> Self {
    Self
  }

  /// Read a PNG image from a file
  pub fn read_file(&mut self, path: &Path) -> Result<PNGImage, String> {
    match fs::read(path) {
      Ok(bytes) => Ok(self.read_bytes(&bytes)?),
      Err(_) => Err(format!("")),
    }
  }

  /// Read a PNG image from a sequence of bytes
  pub fn read_bytes(&mut self, bytes: &[u8]) -> Result<PNGImage, String> {
    let mut reader: PNGReader<'_> = PNGReader::new();
    let image = reader.read_bytes(bytes)?;
    Ok(image)
  }
}
