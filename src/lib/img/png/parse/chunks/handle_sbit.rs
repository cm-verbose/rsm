use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_color_type::ColorType, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle sBIT (Significant bits) Chunk
  pub(in super::super) fn handle_sbit<'a>(
    &self,
    chunk: &Chunk<'a>,
    color_type: ColorType,
  ) -> Result<&'a [u8], RSMError> {
    let range_end: usize = match color_type {
      // Color type: 0
      ColorType::Greyscale => 1,

      // Color type: 2, 3
      ColorType::Truecolor | ColorType::IndexedColor => 3,

      // Color type: 4
      ColorType::GreyscaleAlpha => 2,

      // Color type: 6
      ColorType::TruecolorAlpha => 6,
    };
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
  fn test_sbit_mapping() {
    let data = &[1, 2, 3, 4, 5, 6];
    let data_len = data.len();

    let chunk: Chunk<'_> = Chunk {
      length: data_len as u32,
      r#type: ChunkType::tRNS,
      data,
      crc: [0, 0, 0, 0],
    };

    let expected: [(ColorType, usize); 5] = [
      (ColorType::Greyscale, 1),
      (ColorType::GreyscaleAlpha, 2),
      (ColorType::Truecolor, 3),
      (ColorType::TruecolorAlpha, 6),
      (ColorType::IndexedColor, 3),
    ];
    let parser: PNGParser = PNGParser::new();

    for pair in expected {
      let res = parser.handle_sbit(&chunk, pair.0).unwrap();
      assert_eq!(res.len(), pair.1);
    }
  }
}
