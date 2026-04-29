use crate::lib::{
  img::png::read::reader::{
    reader::PNGReader,
    states::png_state::{ReadIHDR, ReadSignature},
  },
  util::err::error::RSMError,
};
use std::marker::PhantomData;

impl<'d> PNGReader<'d, ReadSignature> {
  /// PNG image signature. Corresponds to:
  /// `[0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa]`.
  const SIGNATURE: u64 = 0x89504E470D0A1A0A;

  /// Read and validate the image signature.
  pub(crate) fn read_signature(&mut self) -> Result<PNGReader<'d, ReadIHDR>, RSMError> {
    let bytes: &[u8; 8] = self.take_sized::<8>()?;
    if Self::SIGNATURE == u64::from_be_bytes(*bytes) {
      Ok(PNGReader {
        _state: PhantomData,
        ptr: self.ptr,
        data: self.data,
      })
    } else {
      Err(RSMError::Other(format!("Invalid signature {:?}", bytes)))
    }
  }
}
