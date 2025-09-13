use crate::rsm_lib::img::png::{
  chunk::chunk::Chunk, image::png_image::PNGImage, reader::png_reader::PNGReader,
};
use std::mem;

impl<'a> PNGReader<'a> {
  /// Reads a sequence of bytes from a PNG datastream to generate an image
  pub(in crate::rsm_lib::img::png) fn read_bytes(
    &'a mut self,
    bytes: &'a [u8],
  ) -> Result<PNGImage, String> {
    self.reset(bytes);
    let chunks: &[Chunk<'a>] = &*self.read_chunks()?;
    self.parse(chunks)?;
    Ok(std::mem::take(&mut self.image))
  }

  /// Reset the reader to its initial state
  fn reset(&mut self, bytes: &'a [u8]) -> () {
    if self.is_used {
      let _ = mem::replace(
        self,
        Self {
          bytes,
          ..Self::default()
        },
      );
    } else {
      self.bytes = bytes;
      self.is_used = true;
    }
  }
}
