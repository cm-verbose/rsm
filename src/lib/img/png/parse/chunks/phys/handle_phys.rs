use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::phys::png_physical_dimensions::PhysicalDimensions, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle pHYs (Physical pixel dimensions) chunk
  pub(in super::super::super) fn handle_phys(
    &self,
    chunk: &Chunk,
  ) -> Result<Option<PhysicalDimensions>, RSMError> {
    if let Ok::<[u8; 9], _>(bytes) = chunk.data.try_into() {
      let x = self.get_phys_size(&bytes[0..4]);
      let y = self.get_phys_size(&bytes[4..8]);

      if let (Some(pp_x), Some(pp_y)) = (x, y) {
        let unit: u8 = bytes[8];
        if unit > 1 {
          return Ok(None);
        }
        Ok(Some(PhysicalDimensions {
          pp_x,
          pp_y,
          is_meter: unit == 1,
        }))
      } else {
        Ok(None)
      }
    } else {
      Err(RSMError::InvalidLength)
    }
  }

  /// Get a physical size (width / height)
  fn get_phys_size(&self, data: &[u8]) -> Option<u32> {
    let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
      return None;
    };
    let size: u32 = u32::from_be_bytes(bytes);
    if size == 0 { None } else { Some(size) }
  }
}
