use crate::lib::{
  img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    read::reader::png_reader::PNGReader,
  },
  util::err::rsm_error::RSMError,
};

impl<'r> PNGReader<'r> {
  /// Read a sequence of bytes as a PNG image
  pub fn read(&mut self, bytes: &'r [u8]) -> Result<Vec<Chunk<'r>>, RSMError> {
    self.load_bytes(bytes)?;
    let chunks: Vec<Chunk<'_>> = self.read_chunks()?;
    Ok(chunks)
  }

  /// Read the image chunk by chunk
  pub fn read_chunks(&mut self) -> Result<Vec<Chunk<'r>>, RSMError> {
    let mut chunks: Vec<Chunk<'r>> = Vec::new();
    loop {
      let chunk: Chunk<'r> = self.read_chunk()?;
      if chunk.r#type == ChunkType::IEND {
        break;
      }
      chunks.push(chunk);
    }
    Ok(chunks)
  }

  /// Read a singular chunk
  pub fn read_chunk(&mut self) -> Result<Chunk<'r>, RSMError> {
    let length: u32 = self.read_chunk_length()?;
    let chunk: Chunk<'r> = Chunk {
      length,
      r#type: self.read_chunk_type()?,
      data: self.read_chunk_data(length)?,
      crc: self.read_chunk_crc()?,
    };

    Ok(chunk)
  }

  /// Read the length of a chunk's contents from a sequence of four bytes
  fn read_chunk_length(&mut self) -> Result<u32, RSMError> {
    let bytes: [u8; 4] = self.get_bytes_sized::<4>()?;
    let length = u32::from_be_bytes(bytes);

    if length <= i32::MAX as u32 {
      Ok(length)
    } else {
      Err(RSMError::InvalidLength)
    }
  }

  /// Read a sequence of four bytes as a chunk type
  fn read_chunk_type(&mut self) -> Result<ChunkType, RSMError> {
    let bytes: [u8; 4] = self.get_bytes_sized::<4>()?;
    let type_bytes = u32::from_be_bytes(bytes);

    let r#type: ChunkType = ChunkType::from(type_bytes);
    Ok(r#type)
  }

  /// Read chunk data
  fn read_chunk_data(&mut self, length: u32) -> Result<&'r [u8], RSMError> {
    self.get_bytes(length as usize)
  }

  fn read_chunk_crc(&mut self) -> Result<[u8; 4], RSMError> {
    self.get_bytes_sized::<4>()
  }

  /// Get a sequence of bytes
  fn get_bytes(&mut self, next: usize) -> Result<&'r [u8], RSMError> {
    if let Some(slice) = self.bytes.get(self.ptr..self.ptr + next) {
      self.ptr += next;
      Ok(slice)
    } else {
      Err(RSMError::NotEnoughContent)
    }
  }

  /// Get bytes as a sized array
  fn get_bytes_sized<const N: usize>(&mut self) -> Result<[u8; N], RSMError> {
    let bytes: &[u8] = self.get_bytes(N)?;
    if let Ok(sized) = bytes.try_into() {
      Ok(sized)
    } else {
      Err(RSMError::InvalidContent)
    }
  }

  /// Load bytes within the reader
  fn load_bytes(&mut self, bytes: &'r [u8]) -> Result<(), RSMError> {
    self.validate_signature(bytes)?;
    self.bytes = bytes;
    Ok(())
  }
}
