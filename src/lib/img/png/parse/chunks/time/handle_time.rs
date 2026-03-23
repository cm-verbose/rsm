use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::time::png_time::ModifiedTime, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle tIME (Image last-modification time)
  pub(in super::super::super) fn handle_time(
    &self,
    chunk: &Chunk,
  ) -> Result<Option<ModifiedTime>, RSMError> {
    let Ok::<[u8; 7], _>(data) = chunk.data.try_into() else {
      return Err(RSMError::InvalidLength);
    };

    let year: u16 = u16::from_be_bytes(data[0..2].try_into().unwrap());
    let month = data[2];
    let day = data[3];
    let hour = data[4];
    let minute = data[5];
    let second = data[6];

    if month == 0 || month > 12 || day == 0 || day > 31 || hour > 23 || minute > 59 || second > 60 {
      return Ok(None);
    }

    Ok(Some(ModifiedTime {
      year,
      month,
      day,
      hour,
      minute,
      second,
    }))
  }
}
