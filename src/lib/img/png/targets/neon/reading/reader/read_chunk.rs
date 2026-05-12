use crate::lib::{
  img::png::targets::neon::{
    chunk::chunk::Chunk,
    reading::reader::{reader::PNGReader, states::reader_states::PNGState},
  },
  util::err::error::RSMError,
};

impl<'d, S: PNGState> PNGReader<'d, S> {
  /// Read a [chunk](Chunk) from the current pointer position and increment
  /// this pointer by the combined size of the given chunk.
  pub(crate) fn read_chunk(&mut self) -> Result<Chunk<'d>, RSMError> {
    let combined_read: [u8; 8] = *self.take_sized::<8>()?;
    let chunk_header: u64 = u64::from_be_bytes(combined_read);

    let r#type: u32 = (chunk_header >> 32) as u32;
    let length: u32 = chunk_header as u32;

    let data: &[u8] = self.take(length as usize)?;
    let crc: [u8; 4] = *self.take_sized::<4>()?;
    Ok(Chunk { r#type, data, crc })
  }
}
