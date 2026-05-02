use crate::lib::{
  img::png::parse::chunks::chunk_data::_ChunkData, util::err::error::RSMError,
};

/// IHDR (Image header) content.
#[derive(Debug)]
pub(crate) struct _ImageHeader {
  _width: u32,
  _height: u32,
  _bit_depth: u8,
  _color_type: u8,
  _compression_method: u8,
  _filter_method: u8,
  _interlace_method: u8,
}

impl<'h> _ChunkData<'h> for _ImageHeader {
  /// Assume data is exactly 13 bytes.
  #[inline(always)]
  fn from_bytes(data: &'h [u8]) -> Result<Self, RSMError> {
    let size_u64: u64 = unsafe {
      let ptr: *const u64 = data.as_ptr() as *const u64;
      u64::from_be(ptr.read_unaligned())
    };
    let width: u32 = (size_u64 >> 32) as u32;
    let height: u32 = size_u64 as u32;

    Ok(_ImageHeader {
      _width: width,
      _height: height,
      _bit_depth: unsafe { *data.get_unchecked(8) },
      _color_type: unsafe { *data.get_unchecked(9) },
      _compression_method: unsafe { *data.get_unchecked(10) },
      _filter_method: unsafe { *data.get_unchecked(11) },
      _interlace_method: unsafe { *data.get_unchecked(12) },
    })
  }
}
