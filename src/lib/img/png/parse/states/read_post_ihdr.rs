use std::marker::PhantomData;

use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::ihdr::png_header::PNGHeader,
      png_parser::PNGParser,
      states::{
        data::png_metadata::PNGMetadata,
        png_state::{ReadIDAT, ReadPostIHDR},
      },
    },
  },
  util::err::rsm_error::RSMError,
};

impl<'p> PNGParser<'p, ReadPostIHDR> {
  pub(crate) fn read_post_ihdr(
    mut self,
    header: &PNGHeader,
  ) -> Result<(PNGParser<'p, ReadIDAT>, PNGMetadata, Chunk<'p>), RSMError> {
    let mut meta: PNGMetadata = PNGMetadata::default();

    loop {
      let chunk: Chunk<'p> = self.read_chunk()?;
      match chunk.r#type {
        ChunkType::IDAT => {
          let state: PNGParser<'p, ReadIDAT> = PNGParser {
            reader: self.reader,
            _state: PhantomData,
          };
          return Ok((state, meta, chunk));
        }

        ChunkType::IHDR => return Err(RSMError::InvalidContent),
        ChunkType::IEND => return Err(RSMError::InvalidContent),
        _ => {
          meta.set_data(chunk, header)?;
        }
      }
    }
  }
}
