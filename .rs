impl<'b> PNGHandler<'b> {
  /// Parse the IDAT chunks into pixel data
  fn parse_IDAT(&mut self) -> Result<(), String> {
    let compression_method: u8 = self.image.compression_method.unwrap();

    if compression_method == 0 {
      let bytes: &[u8] = &inflate::decompress_to_vec_zlib(&self.idat_bytes).unwrap();
      let scanlines: Vec<Scanline> = self.read_scanline(bytes)?;
      let decoded_bytes: Vec<Vec<u8>> = self.undo_image_filters(scanlines)?;

      let pixels: Option<Vec<Vec<Box<dyn Color>>>> = self.convert_bytes_to_pixels(decoded_bytes);
      self.image.pixels = pixels;
      Ok(())
    } else {
      Err(format!(
        "Invalid compression method found: method {} is not defined",
        compression_method
      ))
    }
  }

  /// Parsing the scanlines of the image
  fn read_scanline(&mut self, bytes: &[u8]) -> Result<Vec<Scanline>, String> {
    let scanline_length: u32 = self.read_scanline_length()?;

    let mut scanlines: Vec<Scanline> = Vec::new();
    let mut s_ptr: usize = 0;

    for _ in 0..self.image.height.unwrap() {
      let filter_type = bytes[s_ptr];
      s_ptr += 1;

      let pixel_data_length: usize = (scanline_length - 1) as usize;
      let pixel_data: &[u8] = &bytes[s_ptr..s_ptr + pixel_data_length];
      s_ptr += pixel_data_length;

      scanlines.push(Scanline {
        filter_type,
        data: pixel_data.to_vec(),
      });
    }
    Ok(scanlines)
  }

  /// Undo the image filters
  fn undo_image_filters(&mut self, scanlines: Vec<Scanline>) -> Result<Vec<Vec<u8>>, String> {
    let mut unfiltered: Vec<Vec<u8>> = Vec::new();
    let bytes_per_pixel: usize = self.read_bytes_per_pixel()? as usize;

    for mut scanline in scanlines {
      let previous_scanline: Option<Vec<u8>> = if unfiltered.is_empty() {
        None
      } else {
        Some(unfiltered.last().unwrap().clone())
      };
      match scanline.filter_type {
        0 => {}
        1 => self.undo_sub_filter(&mut scanline.data, bytes_per_pixel),
        2 => self.undo_up_filter(&mut scanline.data, previous_scanline),
        3 => self.undo_average_filter(&mut scanline.data, bytes_per_pixel, previous_scanline),
        4 => self.undo_paeth_filter(&mut scanline.data, bytes_per_pixel, previous_scanline),
        _ => return Err(format!("Invalid filter type: {}", scanline.filter_type)),
      }
      unfiltered.push(scanline.data);
    }
    Ok(unfiltered)
  }

  /// Undo the sub filter
  fn undo_sub_filter(&self, data: &mut [u8], bytes_pixels: usize) -> () {
    for i in bytes_pixels..data.len() {
      data[i] = data[i].wrapping_add(data[i - bytes_pixels]);
    }
  }

  /// Undo the up filter
  fn undo_up_filter(&self, data: &mut [u8], previous_scanline: Option<Vec<u8>>) -> () {
    if let Some(prev) = previous_scanline {
      for i in 0..data.len() {
        data[i] = data[i].wrapping_add(prev[i]);
      }
    }
  }

  /// Undo the average filter
  fn undo_average_filter(
    &self,
    data: &mut [u8],
    bytes_pixels: usize,
    previous_scanline: Option<Vec<u8>>,
  ) -> () {
    for i in 0..data.len() {
      let left = if i >= bytes_pixels {
        data[i - bytes_pixels] as u16
      } else {
        0
      };
      let up = if let Some(ref previous) = previous_scanline {
        previous[i] as u16
      } else {
        0
      };
      let average: u8 = ((left + up) / 2) as u8;
      data[i] = data[i].wrapping_add(average);
    }
  }

  fn undo_paeth_filter(
    &self,
    data: &mut [u8],
    bytes_pixels: usize,
    previous_scanline: Option<Vec<u8>>,
  ) {
    for i in 0..data.len() {
      let left = if i >= bytes_pixels {
        data[i - bytes_pixels] as i16
      } else {
        0
      };
      let up: i16 = if let Some(ref prev) = previous_scanline {
        prev[i] as i16
      } else {
        0
      };
      let up_left: i16 = if i >= bytes_pixels && previous_scanline.is_some() {
        previous_scanline.as_ref().unwrap()[i - bytes_pixels] as i16
      } else {
        0
      };
      let predictor: i16 = self.paeth_predictor(left, up, up_left);
      data[i] = data[i].wrapping_add(predictor as u8);
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

  /// Get the amount of channels per color
  fn get_channel_count(&self) -> Result<u32, String> {
    let channel_count: u32 = match self.image.color_type.unwrap() {
      0 | 3 => 1,
      2 => 3,
      4 => 2,
      6 => 4,
      _ => {
        return Err(format!("Invalid channel found"));
      }
    };
    Ok(channel_count)
  }

  fn read_bytes_per_pixel(&self) -> Result<u32, String> {
    let channel_count: u32 = self.get_channel_count()?;
    Ok(((self.image.bit_depth.unwrap() as u32) * channel_count + 7) / 8)
  }

  /// Read the length of every scanline of an image
  fn read_scanline_length(&self) -> Result<u32, String> {
    let channel_count = self.get_channel_count()?;

    let bits_per_pixel: u32 = channel_count * (self.image.bit_depth.unwrap() as u32);
    let bits_per_scanline = self.image.width.unwrap() * bits_per_pixel;
    Ok(1 + ((bits_per_scanline + 7) / 8))
  }

  /// Convert bytes to pixels
  fn convert_bytes_to_pixels(&self, data: Vec<Vec<u8>>) -> Option<Vec<Vec<Box<dyn Color>>>> {
    let colors: Vec<Vec<Box<dyn Color>>> = data
      .into_iter()
      .map(|scanline| self.get_color(scanline).unwrap())
      .collect();

    Some(colors)
  }

  /// Get the color for every pixel
  fn get_color(&self, scanline: Vec<u8>) -> Result<Vec<Box<dyn Color>>, String> {
    let mut parts: Vec<Box<dyn Color>> = Vec::new();
    let color_type: u8 = self.image.color_type.unwrap();
    let bytes_per_pixel: usize = self.read_bytes_per_pixel().unwrap() as usize;

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