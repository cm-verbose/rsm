use std::marker::PhantomData;

use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::{
        idat::{handle_idat::handle_idat, png_pixel_data::PixelData},
        ihdr::png_header::PNGHeader,
      },
      png_parser::PNGParser,
      states::{
        data::png_metadata::PNGMetadata,
        png_state::{ReadIDAT, ReadPostIDAT},
      },
    },
  },
  util::err::rsm_error::RSMError,
};

impl<'p> PNGParser<'p, ReadIDAT> {
  pub(crate) fn read_idat(
    mut self,
    first: &Chunk<'_>,
    header: &PNGHeader,
    meta: &mut PNGMetadata,
  ) -> Result<(PNGParser<'p, ReadPostIDAT>, PixelData), RSMError> {
    let mut idat_bytes: Vec<u8> = Vec::new();
    idat_bytes.extend_from_slice(first.data);

    loop {
      let chunk: Chunk<'_> = self.read_chunk()?;

      match chunk.r#type {
        ChunkType::IDAT => idat_bytes.extend_from_slice(chunk.data),

        ChunkType::IHDR => return Err(RSMError::InvalidContent),
        ChunkType::PLTE => return Err(RSMError::InvalidContent),

        _ => {
          let pixel_data: PixelData = handle_idat(&idat_bytes, header, meta)?;
          meta.set_data(chunk, header)?;

          return Ok((
            PNGParser {
              reader: self.reader,
              _state: PhantomData,
            },
            pixel_data,
          ));
        }
      }
    }
  }
}
