use crate::lib::{
  img::png::read::reader::{reader::PNGReader, states::png_state::ReadSignature},
  util::err::error::RSMError,
};

impl<'d> PNGReader<'d, ReadSignature> {
  /// Read a PNG image.
  pub(crate) fn read(mut self, data: &'d [u8]) -> Result<(), RSMError> {
    self.data = data;
    let mut next: PNGReader<'_, _> = self.read_signature()?;
    let next: PNGReader<'_, _> = next.read_ihdr()?;
    next.read_post_ihdr();
    Ok(())
  }
}
