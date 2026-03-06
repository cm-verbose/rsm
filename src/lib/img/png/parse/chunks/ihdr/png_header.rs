use crate::lib::img::png::parse::chunks::ihdr::{
  png_bit_depth::BitDepth, png_color_type::ColorType, png_compression_method::CompressionMethod,
  png_filter_method::FilterMethod, png_interlace_method::InterlaceMethod,
};

/// PNG IHDR Header
#[derive(Debug)]
pub struct PNGHeader {
  pub(super) width: u32,
  pub(super) height: u32,
  pub(super) bit_depth: BitDepth,
  pub(super) compression_method: CompressionMethod,
  pub(super) color_type: ColorType,
  pub(super) filter_method: FilterMethod,
  pub(super) interlace_method: InterlaceMethod,
}
