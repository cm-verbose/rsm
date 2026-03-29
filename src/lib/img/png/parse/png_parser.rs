use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::states::png_state::{PNGState, ReadSignature},
    reader::png_reader::PNGReader,
  },
  util::err::rsm_error::RSMError,
};
use std::marker::PhantomData;

/// PNG image parser.
pub struct PNGParser<'p, S: PNGState> {
  pub(crate) reader: PNGReader<'p>,
  pub(crate) _state: PhantomData<S>,
}

impl<'p, S: PNGState> PNGParser<'p, S> {
  /// Read a chunk
  pub(crate) fn read_chunk(&mut self) -> Result<Chunk<'p>, RSMError> {
    let length_bytes: [u8; 4] = *self.reader.take_sized::<4>()?;
    let length: u32 = u32::from_be_bytes(length_bytes);

    if length > (i32::MAX as u32) {
      return Err(RSMError::OutOfBounds);
    }

    let chunk_type_bytes: [u8; 4] = *self.reader.take_sized::<4>()?;
    let chunk_u32: u32 = u32::from_be_bytes(chunk_type_bytes);
    let r#type: ChunkType = chunk_u32.into();

    let data: &[u8] = self.reader.take(length as usize)?;
    let crc: [u8; 4] = *self.reader.take_sized::<4>()?;

    Ok(Chunk {
      length,
      r#type,
      data,
      crc,
    })
  }
}

impl<'p> PNGParser<'p, ReadSignature> {
  /// Create a new PNG image parser
  pub fn new(bytes: &'p [u8]) -> Self {
    Self {
      reader: PNGReader::new(bytes),
      _state: PhantomData,
    }
  }
}
