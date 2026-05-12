use crate::lib::img::png::targets::neon::parsing::idhr::handle_ihdr::ImageHeader;

/// Representation of a PNG
#[derive(Debug)]
pub struct PNGImage {
  header: ImageHeader,
}
