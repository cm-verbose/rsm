use crate::lib::img::png::parse::chunks::ihdr::{
  png_bit_depth::BitDepth, png_color_type::ColorType, png_compression_method::CompressionMethod,
  png_filter_method::FilterMethod, png_interlace_method::InterlaceMethod,
};

/// PNG IHDR Header
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub(in super::super::super) struct PNGHeader {
  pub width: u32,
  pub height: u32,
  pub bit_depth: BitDepth,
  pub compression_method: CompressionMethod,
  pub color_type: ColorType,
  pub filter_method: FilterMethod,
  pub interlace_method: InterlaceMethod,
}
