use crate::rsm_lib::img::png::{
  chunk::chunk::{Chunk, ChunkType},
  reader::png_reader::PNGReader,
};

impl<'a> PNGReader<'a> {
  /// Read bytes and return a vector of chunks
  pub(in crate::rsm_lib::img::png) fn read_chunks(&mut self) -> Result<Box<[Chunk<'a>]>, String> {
    self.validate_signature()?;
    let mut chunks: Vec<Chunk<'a>> = Vec::new();
    let mut idat_bytes: Vec<&[u8]> = Vec::new();

    while let Ok(chunk) = self.read_chunk() {
      match chunk.r#type {
        ChunkType::IEND => {
          chunks.push(chunk);
          break;
        }
        ChunkType::IDAT => {
          idat_bytes.push(chunk.data);
        }
        _ => {}
      }
      chunks.push(chunk);
    }

    self.idat_bytes = idat_bytes.concat();
    let boxed: Box<[Chunk<'a>]> = chunks.into_boxed_slice();
    Ok(boxed)
  }

  /// Read a singular chunk
  fn read_chunk(&mut self) -> Result<Chunk<'a>, String> {
    let length: u32 = self.read_chunk_length()?;
    let r#type: ChunkType = self.read_chunk_type()?;
    let data: &'a [u8] = self.get_bytes(length as usize)?;

    Ok(Chunk {
      length,
      r#type,
      data,
      crc: self.read_chunk_crc()?,
    })
  }

  /// Read the chunk data's length
  fn read_chunk_length(&mut self) -> Result<u32, String> {
    let bytes: [u8; 4] = self.get_bytes_sized::<4>()?;
    let length: u32 = u32::from_be_bytes(bytes);

    if length < i32::MAX as u32 {
      Ok(length)
    } else {
      Err(format!("Invalid length read for chunk: {}", length))
    }
  }

  /// Read the chunk type as a [[ChunkType]]
  fn read_chunk_type(&mut self) -> Result<ChunkType, String> {
    let bytes: [u8; 4] = self.get_bytes_sized::<4>()?;

    for byte in bytes {
      if !byte.is_ascii_alphabetic() {
        return Err(format!("Failed reading chunk type"));
      }
    }
    let byte_value = u32::from_be_bytes(bytes);
    let chunk_type = ChunkType::from(byte_value);
    Ok(chunk_type)
  }

  /// Read the chunk CRC
  fn read_chunk_crc(&mut self) -> Result<[u8; 4], String> {
    let crc: [u8; 4] = self.get_bytes_sized::<4>()?;
    Ok(crc)
  }

  /// Get next n bytes and output a &[[u8]] if the result is sucessful
  /// or output an error otherwise
  fn get_bytes(&mut self, n: usize) -> Result<&'a [u8], String> {
    let end_pos = self.ptr + n;
    let bytes: Result<&[u8], String> = if end_pos <= self.bytes.len() {
      Ok(&self.bytes[self.ptr..(end_pos)])
    } else {
      Err(format!("Failed reading next {} bytes", n))
    };
    self.ptr = end_pos;
    bytes
  }

  /// Get next T bytes and output a &[[u8]; T] if the result is scuessful
  /// or output an error otherwise
  fn get_bytes_sized<const T: usize>(&mut self) -> Result<[u8; T], String> {
    let sized_bytes: [u8; T] = self.get_bytes(T)?.try_into().unwrap();
    Ok(sized_bytes)
  }
}
