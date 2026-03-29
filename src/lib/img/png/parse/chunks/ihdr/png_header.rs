use crate::lib::img::png::parse::chunks::ihdr::{
  png_bit_depth::BitDepth, png_color_type::ColorType, png_compression_method::CompressionMethod,
  png_filter_method::FilterMethod, png_interlace_method::InterlaceMethod,
};

/// PNG Image header representation
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PNGHeader {
  /// Width of the image in pixels
  pub width: u32,

  /// Height of the image in pixels
  pub height: u32,

  /// [Bit depth](BitDepth) of the image
  pub bit_depth: BitDepth,

  /// [Compression method](CompressionMethod) used to compress the image.
  pub compression_method: CompressionMethod,

  /// [Color type](ColorType) of the image
  pub color_type: ColorType,

  /// [Filter method](FilterMethod) of the image
  pub filter_method: FilterMethod,

  /// [Interlace method](InterlaceMethod) applied to the image
  pub interlace_method: InterlaceMethod,
}
