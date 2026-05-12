use crate::lib::{
  img::png::targets::neon::reading::reader::{
    reader::PNGReader, states::reader_states::ReadPrelude,
  },
  util::err::error::RSMError,
};

impl<'d> PNGReader<'d, ReadPrelude> {
  pub(crate) fn read(mut self) -> Result<(), RSMError> {
    let mut next: PNGReader<'_, _> = self.read_prelude()?;
    next.read_header_data()?;
    Ok(())
  }
}
