use crate::rsm::img::png::{
  chunk::chunk::Chunk, image::png_image::PNGImage, reader::png_reader::PNGReader,
};
use std::mem;

impl<'a> PNGReader<'a> {
  /// Read a sequence of bytes into PNG image data
  pub(in super::super) fn read(&mut self, bytes: &'a [u8]) -> Result<PNGImage, String> {
    self.reset(bytes);
    let chunks: Box<[Chunk<'a>]> = self.read_chunks()?;
    self.parse(&*chunks)?;
    Ok(std::mem::take(&mut self.image))
  }

  /// Set or reset the state of the reader
  fn reset(&mut self, bytes: &'a [u8]) {
    if self.used {
      let _ = mem::replace(
        self,
        Self {
          bytes,
          ptr: bytes.as_ptr(),
          ptr_end: unsafe { self.ptr.add(bytes.len()) },
          ..Self::default()
        },
      );
    } else {
      self.bytes = bytes;
      self.used = true;
      self.ptr = bytes.as_ptr();
      self.ptr_end = unsafe { self.ptr.add(bytes.len()) }
    }
  }
}
