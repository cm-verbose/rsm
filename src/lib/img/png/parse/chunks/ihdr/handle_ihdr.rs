use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_header::PNGHeader, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle IHDR (Image header) chunk
  pub(in super::super::super) fn handle_ihdr(
    &self,
    chunk: &Chunk<'_>,
  ) -> Result<PNGHeader, RSMError> {
    if let Ok::<[u8; 13], _>(data) = chunk.data.try_into() {
      let width: u32 = self.get_ihdr_size(&data[0..4])?;
      let height: u32 = self.get_ihdr_size(&data[4..8])?;

      Ok(PNGHeader {
        width,
        height,
        bit_depth: data[8].try_into()?,
        color_type: data[9].try_into()?,
        compression_method: data[10].try_into()?,
        filter_method: data[11].try_into()?,
        interlace_method: data[12].try_into()?,
      })
    } else {
      Err(RSMError::InvalidContent)
    }
  }

  /// Get IHDR size (width or height)
  fn get_ihdr_size(&self, data: &[u8]) -> Result<u32, RSMError> {
    let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
      return Err(RSMError::InvalidContent);
    };
    let size: u32 = u32::from_be_bytes(bytes);
    if size == 0 {
      return Err(RSMError::InvalidLength);
    }
    Ok(size)
  }
}
