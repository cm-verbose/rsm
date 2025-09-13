use crate::rsm_lib::img::png::reader::png_reader::PNGReader;

impl<'a> PNGReader<'a> {
  /// Handle the IEND chunk, handle parsing IDAT chunks 
  pub(in super::super) fn handle_iend(&mut self) -> Result<(), String> {
    self.handle_idat()
  }
}
