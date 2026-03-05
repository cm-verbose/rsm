use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk, img::png_image::PNGImage, read::reader::png_reader::PNGReader,
  },
  util::{data::file_data::FileData, err::rsm_error::RSMError},
};

impl<'i> PNGImage<'i> {
  pub fn read(&mut self, data: &'i FileData) -> Result<(), RSMError> {
    self.read_bytes(data.as_bytes())
  }

  /// Read a file from a sequence of bytes
  pub fn read_bytes(&mut self, mut bytes: &'i [u8]) -> Result<(), RSMError> {
    bytes = bytes.trim_ascii_start();
    if bytes.is_empty() {
      return Err(RSMError::Empty);
    }
    let mut reader: PNGReader<'i> = PNGReader::new();
    let chunks: Vec<Chunk<'i>> = reader.read(bytes)?;
    self.chunks = chunks;

    println!("{:?}", self.chunks);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::PNGImage;

  /// Tests reading raw bytes
  #[test]
  fn read_bytes() {
    let mut image: PNGImage = PNGImage::new();
    assert!(image.read_bytes(&[]).is_err())
  }
}
