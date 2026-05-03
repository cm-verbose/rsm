use std::{
  arch::aarch64::{uint8x16_t, vceqq_u8, vld1q_u8, vminvq_u8},
  intrinsics::likely,
  marker::PhantomData,
};

use crate::lib::{
  img::png::read::reader::neon::{
    reader::PNGReader,
    states::states::{ReadHeaderData, ReadPrelude},
  },
  util::err::{error::RSMError, png_error::PNGError},
};

impl<'d> PNGReader<'d, ReadPrelude> {
  /// Prelude bytes (signature and IHDR invariant).
  const PRELUDE: [u8; 16] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, // [Signature]
    0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, // [Invariant]
  ];

  /// Read the image prelude (signature and IHDR invariant).
  pub(crate) fn read_prelude(
    mut self,
  ) -> Result<(PNGReader<'d, ReadHeaderData>, &'d [u8]), RSMError> {
    let data: &'d [u8; 16] = self.take_sized::<16>()?;

    if likely(Self::validate_prelude(data.as_ptr())) {
      Ok((
        PNGReader {
          _state: PhantomData,
          data: self.data,
          ptr: self.ptr,
        },
        unsafe { &data.get_unchecked(12..) },
      ))
    } else {
      Err(RSMError::PNGError(PNGError::InvalidPrelude))
    }
  }

  /// Validate that the signature and the first IHDR bytes are correctly
  /// defined.
  #[inline(always)]
  fn validate_prelude(prelude: *const u8) -> bool {
    unsafe {
      let expected: uint8x16_t = vld1q_u8(Self::PRELUDE.as_ptr());
      let actual: uint8x16_t = vld1q_u8(prelude);

      let result: uint8x16_t = vceqq_u8(expected, actual);
      vminvq_u8(result) == 0xFF
    }
  }
}
