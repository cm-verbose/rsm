use std::marker::PhantomData;

use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::ihdr::png_header::PNGHeader,
      png_parser::PNGParser,
      states::{
        data::png_metadata::PNGMetadata,
        png_state::{ReadIEND, ReadPostIDAT},
      },
    },
  },
  util::err::rsm_error::RSMError,
};

impl<'p> PNGParser<'p, ReadPostIDAT> {
  pub(crate) fn read_post_idat(
    mut self,
    meta: &mut PNGMetadata,
    header: &PNGHeader,
  ) -> Result<PNGParser<'p, ReadIEND>, RSMError> {
    loop {
      let next: Chunk<'_> = self.read_chunk()?;

      match next.r#type {
        ChunkType::IEND => {
          return Ok(PNGParser {
            reader: self.reader,
            _state: PhantomData,
          });
        }
        _ => meta.set_data(next, header)?,
      };
    }
  }
}
