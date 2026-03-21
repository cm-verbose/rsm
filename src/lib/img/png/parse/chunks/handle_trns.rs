use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_color_type::ColorType, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle tRNS (Transparency) Chunk
  pub(in super::super) fn handle_trns<'a>(
    &self,
    chunk: &Chunk<'a>,
    color_type: ColorType,
  ) -> Result<&'a [u8], RSMError> {
    let range_end: usize = match color_type {
      // Color type: 0
      ColorType::Greyscale => Ok(2),

      // Color type: 2
      ColorType::Truecolor => Ok(6),

      // Color type: 3
      ColorType::IndexedColor => Ok(chunk.data.len()),
      _ => Err(RSMError::InvalidContent),
    }?;
    Self::get_bytes(0..range_end, chunk)
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{chunks::ihdr::png_color_type::ColorType, png_parser::PNGParser},
  };

  #[test]
  fn test_trns_mapping() {
    type Color = ColorType;
    let data = &[1, 2, 3, 4, 5, 6];
    let data_len = data.len();

    let chunk: Chunk<'_> = Chunk {
      length: data_len as u32,
      r#type: ChunkType::tRNS,
      data,
      crc: [0, 0, 0, 0],
    };

    let methods: [(Color, usize); 3] = [
      (Color::Greyscale, 2),
      (Color::Truecolor, 6),
      (Color::IndexedColor, data.len()),
    ];

    let parser: PNGParser = PNGParser::new();

    for test_pair in methods {
      let res = parser.handle_trns(&chunk, test_pair.0).unwrap();
      assert_eq!(res.len(), test_pair.1)
    }
  }

  #[test]
  fn test_invalid_trns() {
    type Color = ColorType;
    let methods: [Color; 2] = [Color::GreyscaleAlpha, Color::TruecolorAlpha];
    let chunk: Chunk<'_> = Chunk {
      length: 2,
      r#type: ChunkType::tRNS,
      data: &[1, 2, 3],
      crc: [0, 0, 0, 0],
    };

    let parser: PNGParser = PNGParser::new();
    for method in methods {
      let res = parser.handle_trns(&chunk, method);
      assert!(res.is_err())
    }
  }
}
