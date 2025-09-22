use crate::rsm::img::png::{
  chunk::chunk::{Chunk, ChunkType},
  reader::png_reader::PNGReader,
};

impl<'a> PNGReader<'a> {
  /// Parses the received chunks into data
  pub(super) fn parse(&mut self, chunks: &[Chunk<'a>]) -> Result<(), String> {
    for chunk in chunks {
      self.parse_chunk(chunk)?;
    }
    Ok(())
  }

  /// Parse a chunk for data
  fn parse_chunk(&mut self, chunk: &Chunk<'a>) -> Result<(), String> {
    match chunk.r#type {
      ChunkType::IEND => self.handle_iend(),
      ChunkType::IHDR => self.handle_ihdr(chunk),
      _ => Ok(()),
    }
  }
}
