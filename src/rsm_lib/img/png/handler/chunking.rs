use crate::rsm_lib::img::png::{
  chunk::png_chunk::{Chunk, ChunkType},
  handler::png_handler::PNGHandler,
};
use std::borrow::Cow;

impl<'b> PNGHandler<'b> {
  /// Resets the internal state of the handler
  pub(super) fn reset(&mut self, bytes: Cow<'b, [u8]>) {
    *self = Self {
      bytes,
      ..Self::default()
    }
  }

  /// Read the chunks from the image
  pub(super) fn handle_chunks(&mut self) -> Result<(), String> {
    loop {
      let chunk: Chunk = self.read_chunk()?;
      if chunk.r#type == ChunkType::IEND {
        self.chunks.push(chunk);
        break;
      }
      if self.ptr >= self.bytes.len() {
        break;
      }
      self.chunks.push(chunk);
    }
    Ok(())
  }

  fn read_chunk(&mut self) -> Result<Chunk, String> {
    let length: u32 = self.read_chunk_length()?;
    let r#type = self.read_chunk_type()?;

    Ok(Chunk {
      length,
      r#type,
      data: self.read_chunk_data(length as usize)?,
      crc: *self.read_chunk_crc()?,
    })
  }

  /// Read the chunk length
  fn read_chunk_length(&mut self) -> Result<u32, String> {
    let bytes: &[u8; 4] = self.get_next_n_bytes::<4>()?;
    let length: u32 = u32::from_be_bytes(*bytes);

    if length > (i32::MAX as u32) {
      Err(format!("Invalid length found : {}", length))
    } else {
      Ok(length)
    }
  }

  /// Read the chunk data
  fn read_chunk_type(&mut self) -> Result<ChunkType, String> {
    let bytes: &[u8; 4] = self.get_next_n_bytes::<4>()?;
    for byte in bytes {
      if !byte.is_ascii_alphabetic() {
        return Err(format!("Failed reading chunk type"));
      }
    }
    let byte_value: u32 = u32::from_be_bytes(*bytes);
    Ok(ChunkType::from(byte_value))
  }

  /// Reads the chunk data from the provided length of the chunk
  fn read_chunk_data(&mut self, length: usize) -> Result<Vec<u8>, String> {
    Ok(self.get_next_bytes(length)?.to_vec())
  }

  /// Reads the CRC part of the chunk
  fn read_chunk_crc(&mut self) -> Result<&[u8; 4], String> {
    Ok(self.get_next_n_bytes::<4>()?)
  }

  /// Read the next n bytes as a `&[u8]`
  fn get_next_bytes(&mut self, n: usize) -> Result<&[u8], String> {
    let bytes: Result<&[u8], String> = if self.ptr + n <= self.bytes.len() {
      Ok(&self.bytes[self.ptr..(self.ptr + n)])
    } else {
      Err(format!("Failed reading next {} bytes", n))
    };
    self.ptr += n;
    bytes
  }

  /// Read the next N bytes as `&[u8; N]` through type conversion
  fn get_next_n_bytes<const N: usize>(&mut self) -> Result<&[u8; N], String> {
    let bytes: &[u8] = self.get_next_bytes(N)?;
    bytes
      .try_into()
      .map_err(|_| format!("Failed reading next 4 bytes"))
  }
}
