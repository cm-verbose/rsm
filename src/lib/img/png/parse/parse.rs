use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{
      chunks::{
        ihdr::png_header::PNGHeader, phys::png_physical_dimensions::PhysicalDimensions,
        srgb::png_rendering_intent::RenderingIntent,
      },
      png_parser::PNGParser,
    },
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
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
        // Handle duplicate IHDR chunks
        ChunkType::IHDR => return Err(RSMError::InvalidContent),
        ChunkType::PLTE => {
          let palette: Vec<[u8; 3]> = self.handle_plte(chunk)?;
          self.palette = Some(palette);
        }

        ChunkType::IDAT => {
          self.parsed_idat = true;
          self.idat_bytes.extend(chunk.data);
        }

        ChunkType::bKGD => {
          if let Some(header) = &self.image_header {
            let bytes: &[u8] = self.handle_bkgd(chunk, header.color_type)?;
            self.background_bytes = Some(bytes.to_vec());
          } else {
            return Err(RSMError::InvalidContent);
          }
        }

        ChunkType::gAMA => {
          let gamma: f32 = self.handle_gama(chunk)?;
          self.gamma = Some(gamma);
        }

        ChunkType::pHYs => {
          let dimensions: Option<PhysicalDimensions> = self.handle_phys(chunk)?;
          self.physical_dimensions = dimensions;
        }

        ChunkType::sBIT => {
          if let Some(header) = &self.image_header {
            let bytes: &[u8] = self.handle_sbit(chunk, header.color_type)?;
            self.significant_bits = Some(bytes.to_vec())
          } else {
            return Err(RSMError::InvalidContent);
          }
        }

        ChunkType::sRGB => {
          let intent: RenderingIntent = self.handle_srgb(chunk)?;
          self.rendering_intent = Some(intent);
        }

        ChunkType::tRNS => {
          if let Some(header) = &self.image_header {
            let bytes: &[u8] = self.handle_trns(chunk, header.color_type)?;
            self.transparency_bytes = Some(bytes.to_vec())
          } else {
            return Err(RSMError::InvalidContent);
          }
        }

        // Private / unhandled chunks
        _ => {
          println!("{:?}", chunk.r#type)
        }
      }
    }
    if !self.parsed_idat {
      return Err(RSMError::InvalidContent);
    }
    self.map_png_data();
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::lib::{
    img::png::{
      chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
      parse::png_parser::PNGParser,
    },
    util::err::rsm_error::RSMError,
  };

  /// Test empty chunks
  #[test]
  fn test_empty() {
    let mut parser: PNGParser = PNGParser::new();
    let parse_result: Result<(), RSMError> = parser.parse(&vec![]);

    assert!(parse_result.is_err())
  }

  /// Handle a situation where IHDR is not the first chunk
  #[test]
  fn test_idhr_not_first() {
    let mut parser: PNGParser = PNGParser::new();
    let parse_result: Result<(), RSMError> = parser.parse(&vec![Chunk {
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
    let mut parser: PNGParser = PNGParser::new();

    let parse_result: Result<(), RSMError> = parser.parse(&vec![
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
