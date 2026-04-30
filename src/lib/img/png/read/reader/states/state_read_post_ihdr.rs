use crate::lib::img::png::read::reader::{
  reader::PNGReader, states::png_state::ReadPostIHDR,
};

impl<'d> PNGReader<'d, ReadPostIHDR> {
  pub(crate) fn read_post_ihdr(&self) {}
}
