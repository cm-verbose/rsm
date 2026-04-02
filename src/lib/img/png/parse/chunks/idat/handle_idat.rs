use crate::lib::{
  img::png::parse::{
    chunks::{
      idat::{
        png_filters::FilterType,
        png_pixel_data::PixelData,
        png_subimage::SubImage,
        png_unfilter::{unfilter_average, unfilter_paeth, unfilter_sub, unfilter_up},
      },
      ihdr::{
        png_color_type::ColorType, png_header::PNGHeader, png_interlace_method::InterlaceMethod,
      },
    },
    states::data::png_metadata::PNGMetadata,
  },
  util::err::rsm_error::RSMError,
};
use libdeflater::Decompressor;

/// Handle the IDAT (Image data) chunk
pub(crate) fn handle_idat(
  data: &[u8],
  header: &PNGHeader,
  meta: &PNGMetadata,
) -> Result<PixelData, RSMError> {
  let (mut decompressed, images) = decompress_data(data, header)?;
  handle_scanlines(&mut decompressed, &images, header)?;

  let scanline_bytes: Vec<&[u8]> = handle_bytes(&decompressed, &images);
  let pixels: Vec<u8> = map_pixels(&scanline_bytes, &images, header, meta);

  let pixel_data: PixelData = PixelData {
    data: pixels,
    width: *header.width,
    height: *header.height,
  };
  Ok(pixel_data)
}

/// Decompress Deflate compressed data from the IDAT chunk
fn decompress_data(data: &[u8], header: &PNGHeader) -> Result<(Vec<u8>, Vec<SubImage>), RSMError> {
  let mut subimages: Vec<SubImage> = Vec::new();

  let expected_size: usize = match header.interlace_method {
    InterlaceMethod::Null => {
      let row_bytes = get_bytes_per_scanline(*header.width, header);
      let buffer_length = (*header.height * row_bytes) as usize;

      subimages.push(SubImage {
        width: *header.width,
        height: *header.height,
        bytes_per_scanline: row_bytes,
        buffer_offset: 0,
        buffer_length,
        x_step: 1,
        y_step: 1,
        x_start: 0,
        y_start: 0,
      });

      buffer_length
    }
    InterlaceMethod::Adam7 => {
      let (size, images) = handle_adam7(header);
      subimages.extend(images);
      size
    }
  };

  let mut decompressed: Vec<u8> = vec![0u8; expected_size];
  let mut decompressor: Decompressor = Decompressor::new();

  decompressor
    .zlib_decompress(data, &mut decompressed)
    .map_err(|_| RSMError::DecompressionError)?;

  Ok((decompressed, subimages))
}

/// Compute the bits per color channel based on the [color type](ColorType) per
/// pixel, given by the image header.
fn get_channels_per_pixels(header: &PNGHeader) -> u32 {
  match header.color_type {
    // one channel (gray) / 1 channel (index)
    ColorType::Greyscale | ColorType::IndexedColor => 1,

    // two chanels (gray, alpha)
    ColorType::GreyscaleAlpha => 2,

    // three channels (red, blue, green)
    ColorType::Truecolor => 3,

    // four channels (red, blue, green, alpha)
    ColorType::TruecolorAlpha => 4,
  }
}

/// Compute the amount of bytes per scanline depending on the width of an image
/// or the width of multiple subimages.
fn get_bytes_per_scanline(width: u32, header: &PNGHeader) -> u32 {
  let depth: u32 = header.bit_depth as u32;

  let bpp: u32 = get_channels_per_pixels(header) * depth;
  let total_bits = bpp * width;
  let bytes_per_row: u32 = total_bits.div_ceil(8);

  // Add 1 for the filter byte at the start of a scanline
  1 + bytes_per_row
}

/// Handle Adam7 information for parsing
fn handle_adam7(header: &PNGHeader) -> (usize, Vec<SubImage>) {
  let mut offset: usize = 0;
  let mut images: Vec<SubImage> = Vec::new();

  // 8x8 interlacing grid pattern
  const STARTING_ROW: [u8; 7] = [0, 0, 4, 0, 2, 0, 1];
  const STARTING_COL: [u8; 7] = [0, 4, 0, 2, 0, 1, 0];
  const ROW_INCREMENT: [u8; 7] = [8, 8, 8, 4, 4, 2, 2];
  const COL_INCREMENT: [u8; 7] = [8, 8, 4, 4, 2, 2, 1];

  for pass in 0..7 {
    let mut pass_width = 0;
    let mut pass_height = 0;

    let mut col = STARTING_COL[pass] as u32;
    while col < *header.width {
      pass_width += 1;
      col += COL_INCREMENT[pass] as u32;
    }

    let mut row = STARTING_ROW[pass] as u32;
    while row < *header.height {
      pass_height += 1;
      row += ROW_INCREMENT[pass] as u32;
    }

    if pass_width > 0 && pass_height > 0 {
      let bytes_scanline: u32 = get_bytes_per_scanline(pass_width, header);
      let buffer_length = (bytes_scanline * pass_height) as usize;

      images.push(SubImage {
        width: pass_width,
        height: pass_height,
        bytes_per_scanline: bytes_scanline,
        buffer_offset: offset,
        buffer_length,
        x_step: COL_INCREMENT[pass] as u32,
        y_step: ROW_INCREMENT[pass] as u32,
        x_start: STARTING_COL[pass] as u32,
        y_start: STARTING_ROW[pass] as u32,
      });

      offset += buffer_length;
    }
  }

  (offset, images)
}

/// Map the subimages to scanlines and the unfiltering them.
fn handle_scanlines<'a>(
  decompressed: &'a mut [u8],
  images: &Vec<SubImage>,
  header: &PNGHeader,
) -> Result<&'a [u8], RSMError> {
  for image in images {
    // Get pass bytes
    let start_index: usize = image.buffer_offset;
    let end_index: usize = start_index + image.buffer_length;
    let pass: &mut [u8] = &mut decompressed[start_index..end_index];
    let row_size = image.bytes_per_scanline as usize;

    // Remove 1 for the filter byte
    let pixel_bytes_per_row = row_size - 1;
    let mut previous: Vec<u8> = vec![0u8; pixel_bytes_per_row];

    for index in 0..image.height {
      let row_start = (index as usize) * row_size;
      let filter_method: FilterType = pass[row_start].try_into()?;
      let row: &mut [u8] = &mut pass[row_start + 1..row_start + row_size];

      unfilter_scanline(filter_method, row, &previous, header);
      previous.copy_from_slice(row);
    }
  }
  Ok(decompressed)
}

/// Undo the filter on a scanline.
fn unfilter_scanline(method: FilterType, current: &mut [u8], previous: &[u8], header: &PNGHeader) {
  let channels = get_channels_per_pixels(header);
  let bits_per_pixel: u32 = channels * (header.bit_depth as u32);
  let bpp = bits_per_pixel.div_ceil(8).max(1) as usize;

  match method {
    FilterType::None => {}
    FilterType::Sub => unfilter_sub(current, bpp),
    FilterType::Up => unfilter_up(current, previous),
    FilterType::Average => unfilter_average(current, previous, bpp),
    FilterType::Paeth => unfilter_paeth(current, previous, bpp),
  };
}

/// Map unfiltered bytes to raw scanlines for mapping
fn handle_bytes<'a>(bytes: &'a [u8], images: &Vec<SubImage>) -> Vec<&'a [u8]> {
  let mut scanlines: Vec<&[u8]> = Vec::new();

  for image in images {
    let pass_bytes: &[u8] = &bytes[image.buffer_offset..image.buffer_offset + image.buffer_length];
    let row_size = image.bytes_per_scanline as usize;

    for i in 0..image.height {
      let row_start = (i as usize) * row_size;

      let scanline = &pass_bytes[row_start + 1..row_start + row_size];
      scanlines.push(scanline)
    }
  }
  scanlines
}

/// Map scanline bytes to pixels
fn map_pixels(
  bytes: &[&[u8]],
  images: &Vec<SubImage>,
  header: &PNGHeader,
  meta: &PNGMetadata,
) -> Vec<u8> {
  let width = *header.width as usize;
  let height = *header.height as usize;

  let mut canvas: Vec<u8> = vec![0u8; width * height * 4];
  let mut scanline_iter: std::slice::Iter<'_, &[u8]> = bytes.iter();

  for image in images {
    for row_index in 0..(image.height as usize) {
      let current: &[u8] = scanline_iter.next().expect("Invalid data");

      for col_index in 0..(image.width as usize) {
        let cx = (image.x_start as usize) + col_index * (image.x_step as usize);
        let cy = (image.y_start as usize) + row_index * (image.y_step as usize);

        let cpos = (cx + cy * width) * 4;
        let (r, g, b, a) = read_pixel(current, col_index, header, meta);

        canvas[cpos] = r;
        canvas[cpos + 1] = g;
        canvas[cpos + 2] = b;
        canvas[cpos + 3] = a;
      }
    }
  }
  canvas
}

/// Read the pixel value
fn read_pixel(
  bytes: &[u8],
  col_index: usize,
  header: &PNGHeader,
  meta: &PNGMetadata,
) -> (u8, u8, u8, u8) {
  let bit_depth = header.bit_depth as usize;

  // --- SUB-BYTE PACKING (1, 2, 4 bit depth) ---
  if bit_depth < 8 {
    let pixels_per_byte = 8 / bit_depth;
    let byte_idx = col_index / pixels_per_byte;
    let bit_shift = 8 - bit_depth - ((col_index % pixels_per_byte) * bit_depth);
    let mask = (1 << bit_depth) - 1;
    let raw_val = (bytes[byte_idx] >> bit_shift) & mask;

    if header.color_type == ColorType::IndexedColor {
      let [r, g, b] = if let Some(plte) = &meta.palette {
        plte.get(raw_val as usize).copied().unwrap_or([0, 0, 0])
      } else {
        [0, 0, 0]
      };

      let a = if let Some(trns) = &meta.transparency_bytes {
        trns.get(raw_val as usize).copied().unwrap_or(255)
      } else {
        255
      };

      return (r, g, b, a);
    }
    let scaled_val = ((raw_val as u32 * 255) / mask as u32) as u8;
    return (scaled_val, scaled_val, scaled_val, 255);
  }

  let channels = get_channels_per_pixels(header) as usize;
  let bytes_per_channel = bit_depth / 8;
  let pixel_stride = channels * bytes_per_channel;
  let byte_idx = col_index * pixel_stride;

  let r = bytes[byte_idx];

  match header.color_type {
    ColorType::Greyscale => {
      let mut a = 255;
      if let Some(trns) = &meta.transparency_bytes
        && trns.len() >= 2
        && r == trns[1]
      {
        a = 0;
      }
      (r, r, r, a)
    }
    ColorType::GreyscaleAlpha => {
      let a = bytes[byte_idx + bytes_per_channel];
      (r, r, r, a)
    }
    ColorType::Truecolor => {
      let g = bytes[byte_idx + bytes_per_channel];
      let b = bytes[byte_idx + 2 * bytes_per_channel];
      let mut a = 255;

      if let Some(trns) = &meta.transparency_bytes
        && trns.len() >= 6
        && r == trns[1]
        && g == trns[3]
        && b == trns[5]
      {
        a = 0;
      }
      (r, g, b, a)
    }
    ColorType::TruecolorAlpha => {
      let g = bytes[byte_idx + bytes_per_channel];
      let b = bytes[byte_idx + 2 * bytes_per_channel];
      let a = bytes[byte_idx + 3 * bytes_per_channel];
      (r, g, b, a)
    }
    ColorType::IndexedColor => {
      let [pr, pg, pb] = if let Some(plte) = &meta.palette {
        plte.get(r as usize).copied().unwrap_or([0, 0, 0])
      } else {
        [0, 0, 0]
      };

      let pa = if let Some(trns) = &meta.transparency_bytes {
        trns.get(r as usize).copied().unwrap_or(255)
      } else {
        255
      };

      (pr, pg, pb, pa)
    }
  }
}
