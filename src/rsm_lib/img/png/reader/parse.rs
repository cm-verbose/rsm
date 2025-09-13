use crate::rsm_lib::img::png::{
  chunk::chunk::{Chunk, ChunkType},
  reader::png_reader::PNGReader,
};

impl<'a> PNGReader<'a> {
  /// Parses the chunks into data
  pub(in crate::rsm_lib::img::png) fn parse(&mut self, chunks: &[Chunk<'a>]) -> Result<(), String> {
    for chunk in chunks {
      self.parse_chunk(chunk)?;
    }
    Ok(())
  }

  /// Parse a single chunk based on its type
  fn parse_chunk(&mut self, chunk: &Chunk<'a>) -> Result<(), String> {
    match chunk.r#type {
      ChunkType::IHDR => self.handle_ihdr(chunk),
      ChunkType::IDAT => Ok(()),
      ChunkType::IEND => self.handle_iend(),
      _ => Ok(()),
    }
  }
}
