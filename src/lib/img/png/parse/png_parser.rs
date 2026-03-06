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
    Self {
      image_header: None,
      gamma: None,
    }
  }

  /// Parse PNG chunks
  pub fn parse(&mut self, chunks: &Vec<Chunk<'_>>) -> Result<(), RSMError> {
    // IHDR is the first chunk to appear
    if let Some(first) = chunks.first() {
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

#[cfg(test)]
mod test {
  use crate::lib::img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::png_parser::PNGParser,
  };

  /// Test empty chunks
  #[test]
  fn test_empty() {
    let mut parser = PNGParser::new();
    let parse_result = parser.parse(&vec![]);

    assert!(parse_result.is_err())
  }

  /// Handle a situation where IHDR is not the first chunk
  #[test]
  fn test_idhr_not_first() {
    let mut parser = PNGParser::new();
    let parse_result = parser.parse(&vec![Chunk {
      r#type: ChunkType::IDAT,
      length: 4,
      data: &[1, 2, 3, 4],
      crc: [0, 0, 0, 0],
    }]);

    assert!(parse_result.is_err())
  }

  /// Handle a situation where two IHDR chunks are supplied
  #[test]
  fn handle_duplicate_ihdr() {
    let mut parser = PNGParser::new();

    let parse_result = parser.parse(&vec![
      Chunk {
        r#type: ChunkType::IHDR,
        length: 13,
        data: &[0, 0, 0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0],
        crc: [0, 0, 0, 0],
      },
      Chunk {
        r#type: ChunkType::IHDR,
        length: 4,
        data: &[0, 0, 0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0],
        crc: [0, 0, 0, 0],
      },
    ]);
    assert!(parse_result.is_err())
  }
}
