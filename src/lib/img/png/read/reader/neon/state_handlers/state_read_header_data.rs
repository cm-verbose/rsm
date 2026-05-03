use crate::lib::{
  img::png::{
    parse::chunks::{chunk_data::ChunkData, ihdr::header::ImageHeader},
    read::reader::neon::{reader::PNGReader, states::states::ReadHeaderData},
  },
  util::err::error::RSMError,
};
use crc32fast::Hasher;
use std::intrinsics::unlikely;

impl<'d> PNGReader<'d, ReadHeaderData> {
  /// Read the bytes following the invariant prelude
  #[inline(always)]
  pub(crate) fn read_header_data(
    &mut self,
    type_bytes: &[u8],
  ) -> Result<(), RSMError> {
    let data = self.take_sized::<13>()?;
    let _: ImageHeader = ImageHeader::from_bytes(data)?;

    let crc: u32 = u32::from_be_bytes(*self.take_sized::<4>()?);
    let mut hasher: Hasher = Hasher::new();
    hasher.update(type_bytes);
    hasher.update(data);

    if unlikely(hasher.finalize() != crc) {
      return Err(RSMError::Other(format!("Invalid header CRC")));
    }
    Ok(())
  }
}
