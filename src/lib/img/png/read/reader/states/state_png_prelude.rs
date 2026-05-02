use crate::lib::{
  img::png::read::reader::{
    reader::PNGReader,
    states::png_state::{ReadPostPrelude, ReadPrelude},
  },
  util::err::error::RSMError,
};
use cfg_if::cfg_if;
use core::intrinsics::likely;
use std::marker::PhantomData;

impl<'d> PNGReader<'d, ReadPrelude> {
  cfg_if! {
    if #[cfg(target_feature = "neon")] {
      /// Invariant prelude
      const PRELUDE: [u8; 16] = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49,
        0x48, 0x44, 0x52,
      ];

      /// Prelude validation with Neon.
      fn validate_prelude(prelude: *const u8) -> bool {
        unsafe {
          use std::arch::aarch64::{uint8x16_t, vceqq_u8, vld1q_u8, vminvq_u8};
          let expected: uint8x16_t = vld1q_u8(Self::PRELUDE.as_ptr());
          let data: uint8x16_t = vld1q_u8(prelude);

          let result: uint8x16_t = vceqq_u8(expected, data);
          vminvq_u8(result) == 0xFF
        }
      }

      /// Read the image prelude with Neon registers.
      pub(crate) fn read_prelude(&mut self) -> Result<PNGReader<'d, ReadPostPrelude>, RSMError> {
        let prelude = self.take_sized::<16>()?;
        if likely(Self::validate_prelude(prelude.as_ptr())) {
          Ok(PNGReader {
            data: self.data,
            ptr: self.ptr,
            _state: PhantomData,
          })
        } else {
          Err(RSMError::Other(format!("Invalid prelude")))
        }
      }
    } else {
      /// Read the image prelude (combination of the signature and IHDR)
      pub(crate) fn read_prelude(&mut self) {
        println!("hi");
      }
    }
  }
}
