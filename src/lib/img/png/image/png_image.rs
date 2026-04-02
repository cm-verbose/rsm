use crate::lib::img::png::parse::{
  chunks::{idat::png_pixel_data::PixelData, ihdr::png_header::PNGHeader},
  states::data::png_metadata::PNGMetadata,
};

/// Represents a [PNG](https://w3c.github.io/png) image.
#[derive(Debug)]
pub struct PNGImage {
  pub header: PNGHeader,
  pub meta: PNGMetadata,
  pub data: PixelData,
}
