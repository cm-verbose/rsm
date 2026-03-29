use crate::lib::{
  img::png::parse::{
    png_parser::PNGParser,
    states::png_state::{ReadIHDR, ReadSignature},
  },
  util::err::rsm_error::RSMError,
};
use std::marker::PhantomData;

impl<'p> PNGParser<'p, ReadSignature> {
  /// PNG image signature
  const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa];

  /// Validate the PNG image signature
  pub(crate) fn read_signature(mut self) -> Result<PNGParser<'p, ReadIHDR>, RSMError> {
    let bytes: &[u8; 8] = self.reader.take_sized::<8>()?;

    if *bytes == Self::SIGNATURE {
      Ok(PNGParser {
        reader: self.reader,
        _state: PhantomData,
      })
    } else {
      Err(RSMError::InvalidContent)
    }
  }
}
