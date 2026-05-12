use crate::lib::{
  img::png::targets::neon::{
    parsing::idhr::handle_ihdr::ImageHeader,
    reading::reader::{
      reader::PNGReader,
      states::reader_states::{ReadHeaderData, ReadPostIHDR},
    },
  },
  util::err::error::RSMError,
};
use std::marker::PhantomData;

impl<'d> PNGReader<'d, ReadHeaderData> {
  /// Read the remaining 13 bytes in the image header.
  pub(crate) fn read_header_data(
    &mut self,
  ) -> Result<(PNGReader<'d, ReadPostIHDR>, ImageHeader), RSMError> {
    let remaining: &[u8; 13] = self.take_sized::<13>()?;
    let header: ImageHeader = ImageHeader::new(remaining);

    Ok((
      PNGReader {
        _state: PhantomData,
        data: self.data,
        ptr: self.ptr,
        end: self.end,
      },
      header,
    ))
  }
}
