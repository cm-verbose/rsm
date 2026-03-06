use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::chunks::ihdr::png_header::PNGHeader,
  },
  util::err::rsm_error::RSMError,
};

/// Parse chunks into meaningful data
pub struct PNGParser {
  image_header: Option<PNGHeader>,
  gamma: Option<f32>,
}

impl PNGParser {
  pub fn new() -> Self {
    Self { image_header: None, gamma: None }
  }

  /// Parse PNG chunks
  pub fn parse(&mut self, chunks: &Vec<Chunk<'_>>) -> Result<(), RSMError> {
    // IHDR is the first chunk to appear
    if let Some(first) = chunks.get(0) {
      if first.r#type != ChunkType::IHDR {
        return Err(RSMError::InvalidContent);
      }
      let header: PNGHeader = self.handle_ihdr(first)?;
      self.image_header = Some(header);
    } else {
      return Err(RSMError::InvalidContent);
    }

    for chunk in &chunks[1..] {
      match chunk.r#type {
        // Duplicate IHDR chunks
        ChunkType::IHDR => return Err(RSMError::InvalidContent),
        ChunkType::gAMA => {
          let gamma: f32 = self.handle_gama(chunk)?;
          self.gamma = Some(gamma);
        }
        // Private / unhandled chunks
        _ => {
          println!("{:?}", chunk)
        }
      }
    }
    Ok(())
  }
}
