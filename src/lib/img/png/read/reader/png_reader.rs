#[derive(Default)]
pub struct PNGReader<'a> {
  pub(super) bytes: &'a [u8],
}

impl<'a> PNGReader<'a> {
  pub fn new() -> Self {
    PNGReader::default()
  }
}
