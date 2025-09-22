use crate::rsm::img::png::reader::png_reader::PNGReader;

impl<'a> PNGReader<'a> {
  /// Handle the IEND (image trailer) chunk
  pub(in super::super) fn handle_iend(&mut self) -> Result<(), String> {
    self.handle_idat()
  }
}
