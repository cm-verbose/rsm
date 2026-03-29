use crate::lib::{
  img::png::parse::chunks::time::png_time::ModificationTime, util::err::rsm_error::RSMError,
};

/// Handle `tIME` (Image last-modification time)
pub(crate) fn handle_time(data: [u8; 7]) -> Result<Option<ModificationTime>, RSMError> {
  let year: u16 = u16::from_be_bytes(data[0..2].try_into().unwrap());
  let month = data[2];
  let day = data[3];
  let hour = data[4];
  let minute = data[5];
  let second = data[6];

  if month == 0 || month > 12 || day == 0 || day > 31 || hour > 23 || minute > 59 || second > 60 {
    return Ok(None);
  }

  Ok(Some(ModificationTime {
    year,
    month,
    day,
    hour,
    minute,
    second,
  }))
}
