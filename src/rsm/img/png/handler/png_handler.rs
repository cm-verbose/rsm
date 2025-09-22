use crate::rsm::img::png::{image::png_image::PNGImage, reader::png_reader::PNGReader};
use memmap2::{Mmap, MmapOptions};
use std::{error::Error, fs::File, path::Path};

/// Handles PNG images
pub struct PNGHandler;

impl PNGHandler {
  /// Creates a new PNGHandler to work with PNG images
  pub fn new() -> Self {
    Self
  }

  /// Read a file for PNG data
  pub fn read_file(&self, path: &Path) -> Result<PNGImage, Box<dyn Error>> {
    let file: File = File::open(path)?;
    let map: Mmap = unsafe { MmapOptions::new().map(&file)? };

    let bytes: &[u8] = &map[..];
    self.read_bytes(bytes)
  }

  /// Read a sequence of bytes for PNG data
  pub fn read_bytes(&self, bytes: &[u8]) -> Result<PNGImage, Box<dyn Error>> {
    let mut reader: PNGReader<'_> = PNGReader::new();
    let image: PNGImage = reader.read(bytes).unwrap();
    Ok(image)
  }
}
