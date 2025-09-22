use crate::rsm::img::png::{
  chunk::chunk::{Chunk, ChunkType},
  reader::png_reader::PNGReader,
};
use std::slice;

impl<'a> PNGReader<'a> {
  /// Read the chunks in the image
  pub(super) fn read_chunks(&mut self) -> Result<Box<[Chunk<'a>]>, String> {
    self.validate()?;
    let mut chunks: Vec<Chunk<'a>> = Vec::new();
    while let Ok(chunk) = self.read_chunk() {
      match chunk.r#type {
        ChunkType::IEND => {
          chunks.push(chunk);
          break;
        }
        ChunkType::IDAT => {
          self.idat_bytes.extend_from_slice(chunk.data);
        }
        _ => {
          chunks.push(chunk);
        }
      }
    }
    let boxed: Box<[Chunk<'a>]> = chunks.into_boxed_slice();
    Ok(boxed)
  }

  /// Read a singular chunk
  fn read_chunk(&mut self) -> Result<Chunk<'a>, String> {
    let length: u32 = self.get_chunk_length().unwrap();

    let chunk: Chunk<'a> = Chunk {
      length,
      r#type: self.get_chunk_type()?,
      data: self.get_chunk_data(length)?,
      crc: self.get_crc_data()?,
    };

    let chunk_bytes: &[u8; 4] = &chunk.r#type.value().to_be_bytes();
    self.validate_crc(chunk_bytes, chunk.data, chunk.crc)?;
    Ok(chunk)
  }

  /// Get the chunk length
  fn get_chunk_length(&mut self) -> Result<u32, String> {
    let bytes: [u8; 4] = self.get_bytes_size::<4>()?;
    let length = u32::from_be_bytes(bytes);

    if length <= i32::MAX as u32 {
      Ok(length)
    } else {
      Err(format!("Chunk length {} exceeeds maximum", length))
    }
  }

  /// Get chunk type
  fn get_chunk_type(&mut self) -> Result<ChunkType, String> {
    let bytes: [u8; 4] = self.get_bytes_size::<4>()?;
    let chunk_type = u32::from_be_bytes(bytes);

    let r#type: ChunkType = ChunkType::from(chunk_type);
    Ok(r#type)
  }

  /// Get the chunk data
  fn get_chunk_data(&mut self, length: u32) -> Result<&'a [u8], String> {
    Ok(self.get_bytes(length as usize)?)
  }

  /// Get the chunk crc data
  fn get_crc_data(&mut self) -> Result<[u8; 4], String> {
    Ok(self.get_bytes_size::<4>()?)
  }

  /// Reads next n bytes from the provided bytes
  fn get_bytes(&mut self, n: usize) -> Result<&'a [u8], String> {
    if unsafe { self.ptr.add(n) <= self.ptr_end } {
      unsafe {
        let bytes: &[u8] = slice::from_raw_parts(self.ptr, n);
        self.ptr = self.ptr.add(n);
        Ok(bytes)
      }
    } else {
      Err(format!("Failed getting next {} bytes", n))
    }
  }

  /// Read next N bytes from the provided bytes
  fn get_bytes_size<const N: usize>(&mut self) -> Result<[u8; N], String> {
    let bytes: &'a [u8] = self.get_bytes(N)?;
    let mut arr: [u8; N] = [0u8; N];
    arr.copy_from_slice(bytes);
    Ok(arr)
  }

  /// Validate the signature
  fn validate(&mut self) -> Result<(), String> {
    self.validate_signature()?;
    self.ptr = unsafe { self.bytes.as_ptr().add(8) };
    Ok(())
  }
}
