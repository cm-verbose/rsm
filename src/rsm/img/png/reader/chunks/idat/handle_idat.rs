use crate::{
  rgba,
  rsm::{color::colors::rgba::RGBA, img::png::reader::png_reader::PNGReader},
};
use libdeflater::Decompressor;

impl<'a> PNGReader<'a> {
  /// Handle IDAT (Image data) chunks and store it in the image
  pub(in super::super::super) fn handle_idat(&mut self) -> Result<(), String> {
    let method: u8 = self.image.compression_method.unwrap();
    if method == 0 {
      let bytes: &mut [u8] = &mut self.decode_bytes()?;
      let scanlines: &[&mut [u8]] = &self.read_scanlines(bytes)?;
      let pixels: Vec<RGBA> = self.get_pixel_data(scanlines)?;

      self.image.pixels = Some(pixels);
      Ok(())
    } else {
      Err(format!("Invalid compression method: {}", method))
    }
  }

  /// Decoding the IDAT bytes
  fn decode_bytes(&self) -> Result<Vec<u8>, String> {
    let mut decompressor: Decompressor = Decompressor::new();
    let height: u32 = self.image.height.unwrap();
    let expected_size = ((self.get_row_bytes()? + 1) * height) as usize;

    let mut buffer: Vec<u8> = vec![0u8; expected_size];
    let out: usize = decompressor
      .zlib_decompress(&self.idat_bytes, &mut buffer)
      .unwrap();

    if out != expected_size {
      Err(format!("Input output length mismatch (decompress)"))
    } else {
      Ok(buffer)
    }
  }

  /// Get the decompressed size of the pixel data
  fn get_row_bytes(&self) -> Result<u32, String> {
    let bpp: u32 = self.get_bpp()?;
    let width: u32 = self.image.width.unwrap();
    Ok(bpp * width)
  }

  /// Read image scanlines
  fn read_scanlines<'b>(&mut self, bytes: &'b mut [u8]) -> Result<Vec<&'b mut [u8]>, String> {
    let row_bytes: usize = self.get_row_bytes()? as usize;
    let height: usize = self.image.height.unwrap() as usize;
    let mut scanlines: Vec<&mut [u8]> = bytes.chunks_mut(row_bytes + 1).take(height).collect();

    if scanlines.is_empty() {
      return Err(format!("No scanlines"));
    } else {
      self.unfilter_scanline(scanlines[0], None)?;

      for i in 1..scanlines.len() {
        let (prev, current) = scanlines.split_at_mut(i);
        let previous: Option<&[u8]> = Some(&prev[i - 1]);
        let scanline: &mut [u8] = &mut current[0];

        self.unfilter_scanline(scanline, previous)?;
      }
    }
    Ok(scanlines)
  }

  /// Unfilter a scanline to get the actual color values
  fn unfilter_scanline(
    &mut self,
    scanline: &mut [u8],
    previous: Option<&[u8]>,
  ) -> Result<(), String> {
    let filter: u8 = scanline[0];
    let data: &mut [u8] = &mut scanline[1..];
    let bpp: usize = self.get_bpp()? as usize;

    match filter {
      0 => data,
      1 => self.unfilter_sub(data, bpp),
      2 => self.unfilter_up(data, previous),
      3 => self.unfilter_average(data, previous, bpp),
      4 => self.unfilter_paeth(data, previous, bpp),
      _ => return Err(format!("Invalid filter: {}", filter)),
    };
    Ok(())
  }

  /// Obtain the pixel data
  fn get_pixel_data(&self, scanlines: &[&mut [u8]]) -> Result<Vec<RGBA>, String> {
    let width: usize = self.image.width.unwrap() as usize;
    let height: usize = self.image.height.unwrap() as usize;
    let color_type = self.image.color_type.unwrap();
    let bpp = self.get_bpp()? as usize;

    let mut pixels: Vec<RGBA> = Vec::with_capacity(width * height);

    match color_type {
      0 => {
        for scanline in scanlines {
          for chunk in scanline.chunks_exact(bpp) {
            let color = rgba!(chunk[0], chunk[0], chunk[0], 255);
            pixels.push(color);
          }
        }
      }
      2 => {
        for scanline in scanlines {
          for chunk in scanline.chunks_exact(bpp) {
            let color = rgba!(chunk[0], chunk[1], chunk[2], 255);
            pixels.push(color);
          }
        }
      }
      3 => {
        unimplemented!()
      }
      4 => {
        for scanline in scanlines {
          for chunk in scanline.chunks_exact(bpp) {
            let color = rgba!(chunk[0], chunk[0], chunk[0], chunk[1]);
            pixels.push(color);
          }
        }
      }
      5 => {
        for scanline in scanlines {
          for chunk in scanline.chunks_exact(bpp) {
            let color = rgba!(chunk[0], chunk[1], chunk[2], chunk[3]);
            pixels.push(color);
          }
        }
      }
      _ => return Err(format!("Invalid color type")),
    }
    Ok(pixels)
  }

  /// Get the number of channels for each color
  fn get_channels(&self) -> Result<u32, String> {
    let channels: u32 = match self.image.color_type.unwrap() {
      0 | 3 => 1,
      2 => 3,
      4 => 2,
      6 => 4,
      _ => {
        return Err(format!("Invalid color type"));
      }
    };
    Ok(channels)
  }

  /// Get number of bits per pixel
  fn get_bits_per_pixel(&self) -> Result<u32, String> {
    let bit_depth = self.image.bit_depth.unwrap() as u32;
    Ok(self.get_channels()? * bit_depth)
  }

  /// Get the bytes per pixel used in the image
  fn get_bpp(&self) -> Result<u32, String> {
    let bits: u32 = self.get_bits_per_pixel()?;
    let bpp: u32 = (bits + 7) / 8;
    Ok(bpp)
  }
}
