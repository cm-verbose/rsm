/// Defines the PNGImageData struct and it's default values
macro_rules! define_png_image_data {
  ($($field:ident: $field_type:ty),+) => {
    /// Contains all information relative to a PNG image
    #[allow(unused)]
    #[derive(Debug)]
    pub struct PNGImageData {
      $(pub $field: Option<$field_type>,)+
    }
    impl Default for PNGImageData {
      fn default() -> Self {
        Self { $($field: None,)+ }
      }
    }
  }
}

define_png_image_data!(
  bit_depth: u8,
  color_type: u8,
  compression_method: u8,
  filter_method: u8,
  height: u32,
  interlace_method: u8,
  width: u32
);
