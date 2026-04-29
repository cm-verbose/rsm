use crate::lib::{
  img::png::{
    chunk::{chunk::Chunk, chunk_types::CHUNK_TYPE_IHDR},
    read::reader::{reader::PNGReader, states::png_state::ReadIHDR},
  },
  util::err::error::RSMError,
};

impl<'d> PNGReader<'d, ReadIHDR> {
  /// Read IHDR data
  pub(crate) fn read_ihdr(&mut self) -> Result<(), RSMError> {
    let chunk: Chunk<'d> = self.read_chunk()?;

    if chunk.r#type != CHUNK_TYPE_IHDR {
      Err(RSMError::Other(String::from("Invalid chunk type")))
    } else {
      if chunk.data.len() != 13 {
        return Err(RSMError::Other(format!("Invalid length ")));
      }
      Ok(())
    }
  }
}
