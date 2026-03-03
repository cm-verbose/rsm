use crate::lib::{img::png::read::reader::png_reader::PNGReader, util::rsm_error::RSMError};

impl<'a> PNGReader<'a> {
  /// Read a sequence of bytes as a PNG image
  pub fn read(&mut self, bytes: &'a [u8]) -> Result<(), RSMError> {
    self.load_bytes(bytes)?;
    self.read_chunks();
    Ok(())
  }

  /// Read the image chunk by chunk
  pub fn read_chunks(&mut self) {
    println!("reading chunks");
  }

  /// Load bytes within the reader
  fn load_bytes(&mut self, bytes: &'a [u8]) -> Result<(), RSMError> {
    self.validate_signature(bytes)?;
    self.bytes = bytes;
    Ok(())
  }
}
