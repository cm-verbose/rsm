use crate::lib::{
  img::png::read::reader::neon::{reader::PNGReader, states::states::PNGState},
  util::err::{error::RSMError, png_error::PNGError},
};
use std::slice::from_raw_parts;

impl<'d, S: PNGState> PNGReader<'d, S> {
  /// Read *next* amount of bytes from the reader's current position and
  /// increment the reader pointer.
  pub(crate) fn take(&mut self, next: usize) -> Result<&'d [u8], RSMError> {
    let end: usize = self.data.as_ptr() as usize + self.data.len();

    if (self.ptr as usize) + next > end {
      return Err(RSMError::PNGError(PNGError::OutOfBounds));
    }

    unsafe {
      let slice: &[u8] = from_raw_parts(self.ptr, next);
      self.ptr = self.ptr.add(next);
      Ok(slice)
    }
  }

  /// Read a constant **N** amount of bytes from the reader's current position
  /// and increment the reader pointer.
  pub(crate) fn take_sized<const N: usize>(
    &mut self,
  ) -> Result<&'d [u8; N], RSMError> {
    let sized: &'d [u8; N] = self
      .take(N)?
      .try_into()
      .map_err(|_| RSMError::PNGError(PNGError::OutOfBounds))?;
    Ok(sized)
  }
}
