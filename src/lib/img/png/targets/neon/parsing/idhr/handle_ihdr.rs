/// Representation of the IHDR (Image header) chunk
#[derive(Debug)]
pub(crate) struct ImageHeader {
  width: u32,
  height: u32,
  bit_depth: u8,
  color_type: u8,
  compression_method: u8,
  filter_method: u8,
  interlace_method: u8,
}

impl ImageHeader {
  /// Handle parsing the IHDR chunk.
  #[inline(always)]
  pub(crate) fn new(data: &[u8; 13]) -> Self {
    let ptr: *const u8 = data.as_ptr();
    let be_bytes = u32::from_be_bytes;
    type U32ptr = *const [u8; 4];

    unsafe {
      // These run simultaneously
      let width: u32 = be_bytes(*(ptr as U32ptr));
      let height: u32 = be_bytes(*(ptr.add(4) as U32ptr));

      Self {
        width,
        height,
        bit_depth: *data.get_unchecked(8),
        color_type: *data.get_unchecked(9),
        compression_method: *data.get_unchecked(10),
        filter_method: *data.get_unchecked(11),
        interlace_method: *data.get_unchecked(12),
      }
    }
  }
}
