use crate::lib::{
  img::png::read::reader::{reader::PNGReader, states::png_state::ReadSignature},
  util::err::error::RSMError,
};

impl<'d> PNGReader<'d, ReadSignature> {
  /// Parse a sequence of bytes as a PNG image
  pub(crate) fn read(mut self, data: &'d [u8]) -> Result<(), RSMError> {
    self.data = data;
    let mut next: PNGReader<'_, _> = self.read_signature()?;
    next.read_ihdr()?;
    Ok(())
  }
}
