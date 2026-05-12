use crate::lib::img::png::targets::neon::reading::reader::{
  reader::PNGReader, states::reader_states::PNGState,
};

impl<'d, S: PNGState> PNGReader<'d, S> {
  /// Computes the CRC32 value from a given slice.
  pub(crate) fn crc32(&self, data: &'d [u8]) {
    let ptr: *const u8 = data.as_ptr();
    let length: usize = data.len();
  }
}
