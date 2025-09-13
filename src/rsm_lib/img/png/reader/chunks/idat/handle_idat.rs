use crate::{
  rgb, rgba,
  rsm_lib::{color::color::Color, img::png::reader::png_reader::PNGReader},
};
use flate2::bufread::ZlibDecoder;
use std::{error::Error, io::Read};

impl<'a> PNGReader<'a> {
  /// Handle obtaining visual data from the image
  pub(in super::super) fn handle_idat(&mut self) -> Result<(), String> {
    let compression_method: u8 = self.image.compression_method.unwrap();

    if compression_method == 0 {
      let bytes: &mut [u8] = &mut self.decode_bytes().unwrap();
      let scanlines: Vec<Vec<u8>> = self.read_scanlines(bytes)?;
      let pixels: Vec<Box<dyn Color>> = self.convert_bytes_to_pixels(scanlines)?;
      self.image.pixels = Some(pixels);

      Ok(())
    } else {
      Err(format!(
        "Invalid compression method: Compression method {} is not defined",
        compression_method
      ))
    }
  }

  /// Decode IDAT bytes
  fn decode_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decoder: ZlibDecoder<&[u8]> = ZlibDecoder::new(&self.idat_bytes);
    let mut decompressed: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
  }

  /// Read the scanlines from bytes
  fn read_scanlines(&self, bytes: &mut [u8]) -> Result<Vec<Vec<u8>>, String> {
    let row_bytes: usize = self.get_row_bytes()? as usize;
    let height: usize = self.image.height.unwrap() as usize;
    let mut scanlines: Vec<&[u8]> = Vec::with_capacity(height);
    let mut offset: usize = 0;

    // Attempting to loop while unfiltering tends to result in borrow
    // checker errors, hence the process is split in two
    for _ in 0..height {
      let end: usize = offset + row_bytes + 1;
      let scanline: &[u8] = &bytes[offset..end];
      scanlines.push(scanline);

      offset = end;
    }
    let mut unfiltered: Vec<Vec<u8>> = Vec::with_capacity(height);
    let mut previous: Option<&[u8]> = None;

    for scanline in &mut scanlines {
      let mut out: Vec<u8> = vec![0u8; scanline.len() - 1];

      self.unfilter_scanline(scanline, &mut out, previous)?;
      unfiltered.push(out);
      previous = Some(unfiltered.get(0).unwrap());
    }
    Ok(unfiltered)
  }

  /// Unfilters a scanline
  fn unfilter_scanline<'b>(
    &self,
    scanline: &'b [u8],
    out: &'b mut [u8],
    previous: Option<&'b [u8]>,
  ) -> Result<&'b [u8], String> {
    let filter: u8 = scanline[0];
    let bpp = self.get_bpp()? as usize;
    let data: &'b [u8] = &scanline[1..];

    let unfiltered: &'b [u8] = match filter {
      0 => data,
      1 => self.unfilter_sub(data, out, bpp),
      2 => self.unfilter_up(data, out, previous),
      3 => self.unfilter_average(data, out, previous, bpp),
      4 => self.unfilter_paeth(data, out, previous, bpp),
      _ => return Err(format!("Invalid filter method: {}", filter)),
    };

    Ok(unfiltered)
  }

  /// Get the number of bytes per pixel used in the image
  fn get_bits_per_pixel(&self) -> Result<u32, String> {
    let channels: u32 = match self.image.color_type.unwrap() {
      0 | 3 => 1,
      2 => 3,
      4 => 2,
      6 => 4,
      _ => {
        return Err(format!("Invalid color type"));
      }
    };
    let bit_depth: u32 = self.image.bit_depth.unwrap() as u32;
    Ok(channels * bit_depth)
  }

  /// Get the bytes per pixel used in the image
  fn get_bpp(&self) -> Result<u32, String> {
    let bits = self.get_bits_per_pixel()?;
    let bpp = (bits + 7) / 8;
    Ok(bpp)
  }

  /// Get the amount of bytes per scanline (row)
  fn get_row_bytes(&self) -> Result<u32, String> {
    let bits: u32 = self.get_bits_per_pixel()?;
    let row_bytes = (self.image.width.unwrap() * bits + 7) / 8;
    Ok(row_bytes)
  }

  // Convert the bytes to pixel
  fn convert_bytes_to_pixels(
    &self,
    scanlines: Vec<Vec<u8>>,
  ) -> Result<Vec<Box<dyn Color>>, String> {
    let mut colors: Vec<Box<dyn Color>> = Vec::new();

    for scanline in scanlines {
      let row_colors = self.get_colors(scanline)?;
      colors.extend(row_colors);
    }
    Ok(colors)
  }

  fn get_colors(&self, scanline: Vec<u8>) -> Result<Vec<Box<dyn Color>>, String> {
    let mut parts: Vec<Box<dyn Color>> = Vec::new();
    let color_type: u8 = self.image.color_type.unwrap();
    let bytes_per_pixel: usize = self.get_bpp().unwrap() as usize;

    for part in scanline.chunks(bytes_per_pixel) {
      match color_type {
        0 => {
          let grayscale: Box<dyn Color> = Box::new(rgb!(part[0], part[0], part[0]));
          parts.push(grayscale);
        }
        2 => {
          let rgb: Box<dyn Color> = Box::new(rgb!(part[0], part[1], part[2]));
          parts.push(rgb);
        }
        3 => {
          // Handle PLTE
          unimplemented!()
        }
        4 => {
          let grayscale_alpha: Box<dyn Color> = Box::new(rgba!(part[0], part[0], part[0], part[1]));
          parts.push(grayscale_alpha)
        }
        6 => {
          let rgba: Box<dyn Color> = Box::new(rgba!(part[0], part[1], part[2], part[3]));
          parts.push(rgba);
        }
        _ => return Err(format!("Invalid color type found: {}", color_type)),
      }
    }
    Ok(parts)
  }
}
