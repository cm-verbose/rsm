use crate::lib::{
  img::png::targets::neon::reading::reader::{
    reader::PNGReader, states::reader_states::PNGState,
  },
  util::err::{error::RSMError, png_error::PNGError},
};
use std::{intrinsics::unlikely, slice};

impl<'d, S: PNGState> PNGReader<'d, S> {
  /// Read and return the next *n* bytes, from the current pointer position
  /// while also incrementing the pointer by *n*.
  #[inline(always)]
  pub(crate) fn take(&mut self, n: usize) -> Result<&'d [u8], RSMError> {
    if unlikely((self.ptr as usize + n) > self.end as usize) {
      return Err(RSMError::PNGError(PNGError::OutOfBounds));
    }
    unsafe {
      let slice: &'d [u8] = slice::from_raw_parts(self.ptr, n);
      self.ptr = self.ptr.add(n);
      Ok(slice)
    }
  }

  /// Read and return the next *N* bytes, from the current pointer position
  /// while also incrementing the pointer by *N*.
  #[inline(always)]
  pub(crate) fn take_sized<const N: usize>(
    &mut self,
  ) -> Result<&'d [u8; N], RSMError> {
    unsafe {
      let next: *const u8 = self.ptr.add(N);

      if unlikely(next > self.end) {
        return Err(RSMError::PNGError(PNGError::OutOfBounds));
      }
      let data: &'d [u8; N] = &*(self.ptr as *const [u8; N]);
      self.ptr = next;
      Ok(data)
    }
  }
}
