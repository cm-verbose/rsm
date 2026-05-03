use crate::lib::{
  img::png::read::reader::neon::{reader::PNGReader, states::states::ReadPrelude},
  util::err::error::RSMError,
};

impl<'d> PNGReader<'d, ReadPrelude> {
  pub(crate) fn read(mut self, bytes: &'d [u8]) -> Result<(), RSMError> {
    self.data = bytes;
    self.ptr = self.data.as_ptr();

    let (mut next, header_bytes) = self.read_prelude()?;
    next.read_header_data(header_bytes)?;
    Ok(())
  }
}
