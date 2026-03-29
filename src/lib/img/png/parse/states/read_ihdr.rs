use std::marker::PhantomData;

use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::ihdr::{handle_ihdr::handle_ihdr, png_header::PNGHeader},
      png_parser::PNGParser,
      states::png_state::{ReadIHDR, ReadPostIHDR},
    },
  },
  util::err::rsm_error::RSMError,
};

impl<'p> PNGParser<'p, ReadIHDR> {
  pub(crate) fn read_ihdr(mut self) -> Result<(PNGParser<'p, ReadPostIHDR>, PNGHeader), RSMError> {
    let chunk: Chunk<'p> = self.read_chunk()?;

    if chunk.r#type != ChunkType::IHDR {
      return Err(RSMError::InvalidContent);
    }
    let header: PNGHeader = chunk.parse_data_sized::<13, _, _>(|&data| handle_ihdr(data))?;

    Ok((
      PNGParser {
        reader: self.reader,
        _state: PhantomData,
      },
      header,
    ))
  }
}
