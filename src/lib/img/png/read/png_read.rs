use crate::lib::{
  img::png::{img::png_image::PNGImage, read::reader::png_reader::PNGReader},
  util::rsm_error::RSMError,
};
use memmap2::Mmap;
use std::{fs::File, io::Read, path::Path};

impl<'i> PNGImage<'i> {
  /// Read a file from a given path
  pub fn read(&self, path: &Path) -> Result<(), RSMError> {
    let mut file: File = File::open(path)?;
    let file_sz: usize = file.metadata()?.len() as usize;
    let pg_sz: usize = page_size::get();

    if pg_sz * 4 < file_sz {
      let mut buffer: Vec<u8> = Vec::with_capacity(file_sz);
      file.read_to_end(&mut buffer)?;
      self.read_bytes(&buffer)
    } else {
      let map = unsafe { Mmap::map(&file) }?;
      self.read_bytes(&map)
    }
  }

  /// Read a file from a sequence of bytes
  pub fn read_bytes(&self, mut bytes: &[u8]) -> Result<(), RSMError> {
    bytes = bytes.trim_ascii();
    if bytes.is_empty() {
      return Err(RSMError::Empty);
    }
    let mut reader: PNGReader<'_> = PNGReader::new();
    reader.read(bytes)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::PNGImage;

  /// Tests reading raw bytes
  #[test]
  fn read_bytes() {
    let image: PNGImage = PNGImage::new();
    assert!(image.read_bytes(&[]).is_err())
  }
}
