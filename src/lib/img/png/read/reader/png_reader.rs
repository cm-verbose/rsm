/// Struct used for reading PNG content
pub struct PNGReader<'r> {
  pub(super) bytes: &'r [u8],
  pub(super) ptr: usize,
}

impl<'r> Default for PNGReader<'r> {
  fn default() -> Self {
    Self { bytes: &[], ptr: 8 }
  }
}

impl<'r> PNGReader<'r> {
  pub fn new() -> Self {
    Self::default()
  }
}
