use crate::lib::{
  img::png::read::reader::{reader::PNGReader, states::png_state::ReadPrelude},
  util::err::error::RSMError,
};

impl<'d> PNGReader<'d, ReadPrelude> {
  /// Read a PNG image.
  pub(crate) fn read(mut self, data: &'d [u8]) -> Result<(), RSMError> {
    self.data = data;
    self.read_prelude()?;
    Ok(())
  }
}
