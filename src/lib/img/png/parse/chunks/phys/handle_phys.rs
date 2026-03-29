use crate::lib::{
  img::png::parse::chunks::phys::png_physical_dimensions::PhysicalDimensions,
  util::err::rsm_error::RSMError,
};

/// Handle `pHYs` (Physical pixel dimensions) chunk
pub(crate) fn handle_phys(data: [u8; 9]) -> Result<Option<PhysicalDimensions>, RSMError> {
  let x = get_phys_size(&data[0..4]);
  let y = get_phys_size(&data[4..8]);

  if let (Some(pp_x), Some(pp_y)) = (x, y) {
    let unit: u8 = data[8];
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
}

/// Get a physical size (width / height)
fn get_phys_size(data: &[u8]) -> Option<u32> {
  let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
    return None;
  };
  let size: u32 = u32::from_be_bytes(bytes);
  if size == 0 || size > (i32::MAX as u32) {
    None
  } else {
    Some(size)
  }
}
