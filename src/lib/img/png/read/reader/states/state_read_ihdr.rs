use crate::lib::{
  img::png::{
    chunk::{chunk::Chunk, chunk_types::CHUNK_TYPE_IHDR},
    read::reader::{
      reader::PNGReader,
      states::png_state::{ReadIHDR, ReadPostIHDR},
    },
  },
  util::err::error::RSMError,
};
use std::marker::PhantomData;

impl<'d> PNGReader<'d, ReadIHDR> {
  /// Read IHDR data.
  pub(crate) fn read_ihdr(
    &mut self,
  ) -> Result<PNGReader<'d, ReadPostIHDR>, RSMError> {
    let chunk: Chunk<'d> = self.read_chunk()?;

    if chunk.r#type != CHUNK_TYPE_IHDR {
      Err(RSMError::Other(String::from("Invalid chunk type")))
    } else {
      if chunk.data.len() == 13 {
        self.handle_chunk(chunk.r#type, chunk.data, |_, _| Ok(()))?;
        Ok(PNGReader {
          _state: PhantomData,
          data: self.data,
          ptr: self.ptr,
        })
      } else {
        Err(RSMError::Other(format!("Invalid length ")))
      }
    }
  }
}
