/// Struct used for reading PNG content
#[derive(Default)]
pub struct PNGReader<'r> {
  pub(super) bytes: &'r [u8],
}

impl<'r> PNGReader<'r> {
  pub fn new() -> Self {
    PNGReader::default()
  }
}
