use crate::rsm_lib::color::color::Color;

macro_rules! define_png_image {
  ($($field: ident: $field_type: ty),+) => {
    /// Contains the data of a `.png` image
    #[derive(Debug)]
    pub struct PNGImage {
      $(pub $field: Option<$field_type>,)+
    }

    impl Default for PNGImage {
      fn default() -> Self {
        Self {
          $($field: None,)+
        }
      }
    }
  };
}

define_png_image! (
  width: u32,
  height: u32,
  bit_depth: u8,
  color_type: u8,
  compression_method: u8,
  filter_method: u8,
  interlace_method: u8,
  pixels: Vec<Box<dyn Color>>
);
