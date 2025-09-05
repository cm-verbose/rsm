use std::{borrow::Cow, fs, path::Path};

use crate::rsm_lib::{
  img::png::{handler::png_handler::PNGHandler, image::png_image::PNGImage},
  util::reporter::Reporter,
};

impl<'b> PNGHandler<'b> {
  /// Read a file as a `.png` image
  pub fn read_file(&'b mut self, path: &Path) -> Result<&'b PNGImage, String> {
    if !path.exists() {
      return Reporter::report_missing_file(path);
    }
    if path.is_dir() {
      return Reporter::report_dir_instead_of_file(path);
    }

    match fs::read(path) {
      Ok(bytes) => {
        let refs: Cow<'b, [u8]> = Cow::Owned(bytes);
        self.read_bytes(refs)
      }
      Err(err) => Reporter::report_failed_reading_file(path, err),
    }
  }

  /// Read a sequence of bytes
  fn read_bytes(&'b mut self, bytes: Cow<'b, [u8]>) -> Result<&'b PNGImage, String> {
    self.reset(bytes);
    self.validate_signature()?;
    self.handle_chunks()?;
    self.parse_chunks()?;
    Ok(&self.image)
  }
}
