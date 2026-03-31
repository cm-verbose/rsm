use crate::lib::{
  img::png::parse::{
    chunks::phys::png_physical_dimensions::PhysicalDimensions, values::png_int::PNGInt,
  },
  util::err::rsm_error::RSMError,
};

/// Handle `pHYs` (Physical pixel dimensions) chunk.
pub(crate) fn handle_phys(data: [u8; 9]) -> Result<Option<PhysicalDimensions>, RSMError> {
  let x: PNGInt = data[0..4].try_into()?;
  let y: PNGInt = data[4..8].try_into()?;

  let unit: u8 = data[8];
  if unit > 1 {
    return Ok(None);
  }
  Ok(Some(PhysicalDimensions {
    pp_x: *x,
    pp_y: *y,
    is_meter: unit == 1,
  }))
}
