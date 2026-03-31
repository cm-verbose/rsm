use std::marker::PhantomData;

use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::ihdr::png_header::PNGHeader,
      png_parser::PNGParser,
      states::png_state::{ReadIDAT, ReadPostIDAT},
    },
  },
  util::err::rsm_error::RSMError,
};

impl<'p> PNGParser<'p, ReadIDAT> {
  pub(crate) fn read_idat(
    mut self,
    first: &Chunk<'_>,
    header: &PNGHeader,
  ) -> Result<PNGParser<'p, ReadPostIDAT>, RSMError> {
    let mut idat_bytes: Vec<u8> = Vec::new();
    idat_bytes.extend_from_slice(first.data);

    loop {
      let chunk: Chunk<'_> = self.read_chunk()?;

      match chunk.r#type {
        ChunkType::IDAT => idat_bytes.extend_from_slice(chunk.data),

        ChunkType::IHDR => return Err(RSMError::InvalidContent),
        ChunkType::PLTE => return Err(RSMError::InvalidContent),

        _ => {
          return Ok(PNGParser {
            reader: self.reader,
            _state: PhantomData,
          });
        }
      }
    }
  }
}
