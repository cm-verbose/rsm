use std::ops::Range;

use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::chunks::{
      ihdr::png_header::PNGHeader, phys::png_physical_dimensions::PhysicalDimensions,
      srgb::png_rendering_intent::RenderingIntent,
    },
  },
  util::err::rsm_error::RSMError,
};

/// Parse chunks into meaningful data
pub struct PNGParser {
  pub(super) background_bytes: Option<Vec<u8>>,
  pub(super) idat_bytes: Vec<u8>,
  pub(super) image_header: Option<PNGHeader>,
  pub(super) gamma: Option<f32>,
  pub(super) palette: Option<Vec<[u8; 3]>>,
  pub(super) parsed_idat: bool,
  pub(super) physical_dimensions: Option<PhysicalDimensions>,
  pub(super) rendering_intent: Option<RenderingIntent>,
  pub(super) significant_bits: Option<Vec<u8>>,
  pub(super) transparency_bytes: Option<Vec<u8>>,
}

impl PNGParser {
  pub fn new() -> Self {
    Self {
      background_bytes: None,
      image_header: None,
      idat_bytes: Vec::new(),
      gamma: None,
      palette: None,
      parsed_idat: false,
      physical_dimensions: None,
      rendering_intent: None,
      significant_bits: None,
      transparency_bytes: None,
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
    Ok(())
  }

  /// Gets bytes from a range
  pub(super) fn get_bytes<'a>(
    range: Range<usize>,
    chunk: &Chunk<'a>,
  ) -> Result<&'a [u8], RSMError> {
    let Some(bytes) = chunk.data.get(range) else {
      return Err(RSMError::NotEnoughContent);
    };
    Ok(bytes)
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
