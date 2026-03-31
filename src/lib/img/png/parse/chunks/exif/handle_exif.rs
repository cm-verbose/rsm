use crate::lib::{
  img::png::parse::chunks::exif::png_exif::PNGExifData, util::err::rsm_error::RSMError,
};
use exif::{Exif, Reader};
use std::io::Cursor;

/// Handle the `eXIf` chunk
pub(crate) fn handle_exif(data: &[u8]) -> Result<PNGExifData, RSMError> {
  let mut cursor: Cursor<&[u8]> = Cursor::new(data);

  let exif: Exif = Reader::new()
    .read_from_container(&mut cursor)
    .map_err(|_| RSMError::DecompressionError)?;

  let data: PNGExifData = exif.try_into()?;
  Ok(data)
}
