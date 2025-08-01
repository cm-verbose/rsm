use crate::handler::image_handler::handlers::png::{
  chunk_types::ChunkType, png_image_data::PNGImageData,
};
use crate::handler::image_handler::image_handler::ImageParser;
use miniz_oxide::inflate::decompress_to_vec_zlib;

/// https://www.w3.org/TR/png/
pub struct PNGHandler {
  ptr: usize,
  chunks: Vec<Chunk>,
  idat_bytes: Vec<u8>,
  data: PNGImageData,
  bytes: Vec<u8>,
}

/// Represents a PNG's datastream's chunks
#[derive(Clone, Debug)]
struct Chunk {
  length: u32,
  data: Vec<u8>,
  r#type: ChunkType,
  crc: [u8; 4],
}

impl ImageParser for PNGHandler {
  fn new() -> Self {
    Self {
      ptr: 0,
      chunks: Vec::new(),
      idat_bytes: Vec::new(),
      data: PNGImageData::default(),
      bytes: Vec::new(),
    }
  }

  fn parse(&mut self, contents: Vec<u8>) -> Result<(), String> {
    self.parse_contents(contents)?;
    println!("{:?}", self.data);
    return Ok(());
  }
}

impl PNGHandler {
  /// Minimum size of the image because it must contain :
  /// - A signature \[8 bytes]
  /// - IHDR         [4 bytes (type) + 4 bytes (length) + 13 bytes (data) + 4 (crc) => 25 bytes]
  /// - IEND         [4 bytes (type) + 4 bytes (length) +                   4 (crc) => 12 bytes]
  /// Total :        \[45 bytes]
  ///
  const MIN_SIZE: usize = 45;
  const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0xD, 0xA, 0x1A, 0xA];

  fn parse_contents(&mut self, contents: Vec<u8>) -> Result<(), String> {
    self.reset(contents);
    self.validate_signature()?;
    self.parse_chunks()?;
    self.handle_chunks()?;
    Ok(())
  }

  /// Reset the handler
  fn reset(&mut self, bytes: Vec<u8>) {
    self.ptr = 0;
    self.bytes = bytes;
  }

  /// Validating that the image has the appropriate signature
  fn validate_signature(&mut self) -> Result<(), String> {
    if self.bytes.len() < Self::MIN_SIZE {
      return Err(format!("Invalid image length"));
    }
    if self.bytes[0..8] != Self::SIGNATURE {
      return Err(format!(
        "PNG signature does not match, reading: {:?}",
        &self.bytes[0..8]
      ));
    }
    self.ptr += 8;
    Ok(())
  }

  /// Parse the image's chunks
  fn parse_chunks(&mut self) -> Result<(), String> {
    while self.ptr < self.bytes.len() {
      let chunk: Chunk = self.read_chunk()?;
      self.chunks.push(chunk);
    }
    Ok(())
  }

  // Read a PNG image's chunks
  fn read_chunk(&mut self) -> Result<Chunk, String> {
    let length: u32 = self.read_chunk_length_field()?;
    let r#type: ChunkType = self.read_chunk_type_field()?;
    let data: Vec<u8> = self.read_chunk_data_field(length as usize)?;
    let crc: [u8; 4] = *self.read_chunk_crc_field()?;

    if r#type == ChunkType::IDAT {
      self.idat_bytes.append(&mut data.clone());
    }
    Ok(Chunk {
      length,
      data,
      r#type,
      crc,
    })
  }

  // Read the chunk length field
  fn read_chunk_length_field(&mut self) -> Result<u32, String> {
    let length_bytes: [u8; 4] = *self.get_next_n_bytes::<4>()?;
    let length: u32 = u32::from_be_bytes(length_bytes);

    if length > (i32::MAX).try_into().unwrap() {
      Err(format!("Invalid provided length: {}", length))
    } else {
      Ok(length)
    }
  }

  // Read the chunk type
  fn read_chunk_type_field(&mut self) -> Result<ChunkType, String> {
    let type_bytes: [u8; 4] = *self.get_next_n_bytes::<4>()?;
    for byte in type_bytes {
      if !byte.is_ascii_alphabetic() {
        return Err(format!("Invalid byte in chunk type: \"{}\"", byte));
      }
    }
    let byte_value: u32 = u32::from_be_bytes(type_bytes);
    let r#type: ChunkType = ChunkType::from(byte_value);

    Ok(r#type)
  }

  // Read the chunk's data
  fn read_chunk_data_field(&mut self, length: usize) -> Result<Vec<u8>, String> {
    Ok(self.get_next_bytes(length)?.to_vec())
  }

  // Read the chunk's CRC field
  fn read_chunk_crc_field(&mut self) -> Result<&[u8; 4], String> {
    Ok(self.get_next_n_bytes::<4>()?)
  }

  /// Obtain the next N bytes as a `&[u8; N]` or return an error if the bytes are not found.
  /// Then advance pointer by N.
  fn get_next_n_bytes<const N: usize>(&mut self) -> Result<&[u8; N], String> {
    let bytes: Result<&[u8; N], String> = if self.ptr + N <= self.bytes.len() {
      Ok(self.bytes[self.ptr..(self.ptr + N)].try_into().unwrap())
    } else {
      Err(format!("Failed reading next {} bytes", N))
    };
    self.ptr += N;
    bytes
  }

  /// Obtain the next n bytes as a `&[u8]` or return an error if the bytes are not found.
  /// Then advance pointer by n.
  fn get_next_bytes(&mut self, n: usize) -> Result<&[u8], String> {
    let bytes: Result<&[u8], String> = if self.ptr + n <= self.bytes.len() {
      Ok(&self.bytes[self.ptr..(self.ptr + n)])
    } else {
      Err(format!("Failed reading next {} bytes", n))
    };
    self.ptr += n;
    bytes
  }

  /// Handle every chunk
  fn handle_chunks(&mut self) -> Result<(), String> {
    for chunk in self.chunks.to_vec() {
      self.handle_chunk(chunk)?;
    }
    Ok(())
  }

  /// Handles a single chunk
  fn handle_chunk(&mut self, chunk: Chunk) -> Result<(), String> {
    match &chunk.r#type {
      ChunkType::IHDR => self.handle_ihdr(&chunk)?,
      ChunkType::IDAT => {
        return Ok(());
      }
      ChunkType::IEND => {
        self.handle_iend()?;
        return Ok(());
      }
      ChunkType::custom(r#type) => {
        print!("{}", r#type);
      }
      _ => {}
    }
    self.handle_crc(chunk.crc); 
    Ok(())
  }

  fn handle_crc(&self, crc: [u8; 4]){
    println!("{:?}", crc); 
  }

  /// Handle the IHDR chunk
  fn handle_ihdr(&mut self, chunk: &Chunk) -> Result<(), String> {
    if chunk.length != 13 {
      return Err(format!("Invalid IHDR chunk length: {}", chunk.length));
    }

    // Setting width and height
    let width: u32 = u32::from_be_bytes(chunk.data[0..4].try_into().unwrap());
    let height: u32 = u32::from_be_bytes(chunk.data[4..8].try_into().unwrap());

    if height == 0 {
      return Err(format!("Invalid height: 0"));
    }
    if width == 0 {
      return Err(format!("Invalid width: 0"));
    }
    self.data.height = Some(height);
    self.data.width = Some(width);

    // Other fields
    self.data.bit_depth = Some(chunk.data[8]);
    self.data.color_type = Some(chunk.data[9]);
    self.data.compression_method = Some(chunk.data[10]);
    self.data.filter_method = Some(chunk.data[11]);
    self.data.interlace_method = Some(chunk.data[12]);
    return Ok(());
  }

  /// Handle the IEND chunk
  fn handle_iend(&mut self) -> Result<(), String> {
    let bytes: Vec<u8> = decompress_to_vec_zlib(&self.idat_bytes).unwrap();
    let pixels = self.to_rgb_pixels(bytes)?;
    println!("Pixel length: {:?}", pixels.len());
    Ok(())
  }
  
  fn to_rgb_pixels(&mut self, decompressed_idat: Vec<u8>) -> Result<Vec<[u8; 3]>, String> {
    // Unfilter the scanlines
    let unfiltered_bytes = self.unfilter_scanlines(&decompressed_idat)?;

    // Convert RGBA to RGB (your PNG is color type 6 = RGBA)
    let pixels: Vec<[u8; 3]> = unfiltered_bytes
      .chunks_exact(3) // 4 bytes per RGBA pixel
      .map(|chunk| [chunk[0], chunk[1], chunk[2]]) // Drop alpha, keep RGB
      .collect();

    println!("{:?}", pixels[0]);
    Ok(pixels)
  }

  fn unfilter_scanlines(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
    let width = self.data.width.unwrap() as usize;
    let height = self.data.height.unwrap() as usize;
    let bytes_per_pixel = 3; // RGBA
    let scanline_len = 1 + (width * bytes_per_pixel); // 3201 bytes per scanline

    let mut result = Vec::new();
    let mut prev_row: Option<Vec<u8>> = None;

    for y in 0..height {
      let start = y * scanline_len;
      if start >= data.len() {
        return Err(format!("Scanline {} exceeds data bounds", y));
      }
      let filter_type = data[start];
      if filter_type > 4 {
        return Err(format!("Invalid filter type {} at row {}", filter_type, y));
      }
      let mut row: Vec<u8> = data[start + 1..start + scanline_len].to_vec();
      match filter_type {
        0 => {} // None - no filtering
        1 => self.unfilter_sub(&mut row, bytes_per_pixel),
        2 => self.unfilter_up(&mut row, &prev_row),
        3 => self.unfilter_average(&mut row, bytes_per_pixel, &prev_row),
        4 => self.unfilter_paeth(&mut row, bytes_per_pixel, &prev_row),
        _ => unreachable!(), // We already checked above
      }

      result.extend_from_slice(&row);
      prev_row = Some(row);
    }

    Ok(result)
  }

  fn unfilter_sub(&self, row: &mut [u8], bpp: usize) {
    for i in bpp..row.len() {
      row[i] = row[i].wrapping_add(row[i - bpp]);
    }
  }

  fn unfilter_up(&self, row: &mut [u8], prev_row: &Option<Vec<u8>>) {
    if let Some(prev) = prev_row {
      for i in 0..row.len() {
        row[i] = row[i].wrapping_add(prev[i]);
      }
    }
  }

  fn unfilter_average(&self, row: &mut [u8], bpp: usize, prev_row: &Option<Vec<u8>>) {
    for i in 0..row.len() {
      let left = if i >= bpp { row[i - bpp] as u16 } else { 0 };
      let up = if let Some(prev) = prev_row {
        prev[i] as u16
      } else {
        0
      };
      let avg = ((left + up) / 2) as u8;
      row[i] = row[i].wrapping_add(avg);
    }
  }

  fn unfilter_paeth(&self, row: &mut [u8], bpp: usize, prev_row: &Option<Vec<u8>>) {
    for i in 0..row.len() {
      let left = if i >= bpp { row[i - bpp] as i16 } else { 0 };
      let up = if let Some(prev) = prev_row {
        prev[i] as i16
      } else {
        0
      };
      let up_left = if i >= bpp && prev_row.is_some() {
        prev_row.as_ref().unwrap()[i - bpp] as i16
      } else {
        0
      };
      let predictor = self.paeth_predictor(left, up, up_left);
      row[i] = row[i].wrapping_add(predictor as u8);
    }
  }

  fn paeth_predictor(&self, a: i16, b: i16, c: i16) -> i16 {
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();

    if pa <= pb && pa <= pc {
      a
    } else if pb <= pc {
      b
    } else {
      c
    }
  }
}
